# Research Issue: Measuring & Optimizing AI Agent Effectiveness with `did`

## Overview
Conduct research to define measurable metrics, benchmark benchmarks, and evaluation frameworks for assessing how effectively AI agents (such as Claude, Jules, Codex, or AutoGPT) interact with `did` to accomplish software engineering tasks with minimal errors.

---

## Research Goals & Key Questions

### 1. Measurable Performance Metrics
- **Task Resolution Efficiency**: Average number of CLI tool calls required to complete a task.
- **Error & Retry Rate**: Frequency of invalid command invocations (e.g. attempting `did show <directory>` instead of `did status`, or trying `did done` on blocked tasks).
- **Context Overhead**: Token count consumed by reading parent context (`.hooks/show`) and task details (`did show`).
- **Dependency Awareness**: Accuracy of AI agents in recognizing blocked sub-items without making redundant state modifications.

### 2. Telemetry & Benchmark Sandbox
- Design a benchmark suite consisting of standard software engineering tasks managed via `.did/`.
- Measure baseline AI performance across different prompt structures, hook guidance (`.hooks/show`), and command error messages.
- Compare AI error rates before and after error message improvements (e.g. suggesting `did status <path>` when `did show <directory>` is called).

### 3. Fine-Tuning & Prompt Optimization
- Evaluate how `.hooks/show` guidelines influence AI behavior.
- Determine optimal CLI error formatting and suggestions that guide AI models back on track in a single turn.
