# PeakOS & PeakUI Roadmap

This roadmap outlines the coordinated evolution of the PeakOS operating system and the PeakUI framework.

## Phase 1: Foundation & Decoupling (Current)
*Focus: Maturity, genericism, and stability.*

### [PeakUI]
- [x] **Backend Abstraction**: Generic `Backend` trait implemented.
- [x] **Cupertino Refinement**: Premium visual polish (glassmorphism, rounded corners).
- [ ] **Decoupling**: Remove PeakOS-specific types from `peak-ui/src/core.rs`.
- [ ] **Layout Engine**: Improve constraint propagation to prevent "fill vs scroll" paradoxes.

### [PeakOS]
- [x] **Stability**: Resolve core compositor and layout crashes.
- [ ] **System Apps**: Upgrade Settings and File Manager to the new "Premium" look.
- [ ] **Shell Stability**: Ensure smooth transitions between Desktop and Mobile shell modes.

---

## Phase 2: Intelligence & Ecosystem
*Focus: AI Agent integration and developer adoption.*

### [PeakUI]
- [ ] **AI-Native View**: Implement `.describe()` for semantic UI output.
- [ ] **Universal Components**: Release the first set of "Standard Library" widgets.
- [ ] **Documentation**: Complete the "PeakUI Guide" for external developers.

### [PeakOS]
- [ ] **Neural Link**: Integrate PeakIntelligence directly into the Shell for semantic app control.
- [ ] **Store Infrastructure**: First-party app distribution for external PeakUI developers.

---

## Phase 3: Ubiquity
*Focus: Mobile dominance and enterprise readiness.*

### [PeakUI]
- [ ] **Mobile Tooling**: Automated iOS/Android build pipelines.
- [ ] **Visual Inspector**: Real-time layout debugging tool.

### [PeakOS]
- [ ] **Cloud Sync**: Seamless state synchronization across Desktop and Mobile instances.
- [ ] **Enterprise Shell**: Specialized modes for Kiosks and Smart Home displays.
