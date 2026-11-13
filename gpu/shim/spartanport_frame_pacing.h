// SPDX-FileCopyrightText: Copyright 2026 TatsunaKyo
// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once

#include <algorithm>
#include <chrono>
#include <cstdint>
#include <thread>

#ifdef _WIN32
extern "C" void compat_sleep_ns(std::uint64_t ns);
#endif

namespace SpartanFramePacing {
using Clock = std::chrono::steady_clock;

// Keep the phase through ordinary timer jitter. Re-basing every wake-up adds the
// timer's lateness to every frame and turns a 75 FPS limit into roughly 73 FPS.
// Genuine stalls start a fresh interval, so queued frames cannot catch up in bursts.
inline std::chrono::nanoseconds WakeTolerance(std::chrono::nanoseconds period) {
    return std::min(period / 8, std::chrono::nanoseconds{1'000'000});
}

inline Clock::time_point NextFlipDeadline(Clock::time_point previous,
                                          Clock::time_point submitted,
                                          std::chrono::nanoseconds period) {
    if (period.count() <= 0) return submitted;
    // test build: the limiter of windows-v2.7.0-aa-test30 (catch up by at most one period).
    return std::max(previous + period, submitted - period);
}

inline void SleepFor(std::chrono::nanoseconds remaining) {
    if (remaining.count() <= 0) return;
#ifdef _WIN32
    // Reuse the runtime's high-resolution waitable timer instead of a coarse CRT sleep.
    compat_sleep_ns(static_cast<std::uint64_t>(remaining.count()));
#else
    std::this_thread::sleep_for(remaining);
#endif
}

// A timer can wake early (the Windows fallback to Sleep may), so wait out what remains.
inline void SleepUntil(Clock::time_point deadline) {
    // test build: the plain sleep of windows-v2.7.0-aa-test30.
    std::this_thread::sleep_until(deadline);
}
} // namespace SpartanFramePacing
