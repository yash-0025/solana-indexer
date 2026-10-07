# Dual English + Hinglish Explanations & Reference Documentation

## Context & Purpose
Technical blockchain and systems programming concepts in Solana and Rust can easily become dry, tedious, and cognitively exhausting when presented purely in academic jargon. To make learning engaging, intuitive, and fun, the learner prefers explanations in everyday **Hinglish** (Hindi written in Roman/English script, combined naturally with technical English terms).

## Core Directives

1. **Dual Explanation in Chat Responses**:
   - Whenever explaining concepts, system architecture, data flows, code walkthroughs, design trade-offs, exercises, or Rust decisions, ALWAYS provide BOTH:
     - Clear, professional technical English (including domain ELI5 analogies per Rule 8).
     - An engaging, punchy, fun, and crystal-clear **Hinglish technical breakdown** explaining what is actually happening "under the hood" from an engineering perspective.

2. **Knowledgeable + Fun to Read Blend in `hinglish-docs.md`**:
   - Do NOT write dry, academic translations of technical English, and do NOT isolate stories from technical facts.
   - Deliver a seamless blend of **deep technical knowledge + engaging, fun, conversational delivery**:
     - Explain real engineering problems, Solana runtime architecture, 400ms slot constraints, RPC bottlenecks, Borsh/Anchor serialization, memory models, and Rust typing decisions.
     - Frame them with funny, memorable, relatable intuition (e.g. "Solana bullet train hai, RPC bolega 'bhai mere paas time nahi hai'", "Programs bhikhari/stateless hain, sara maal-taal Data Accounts me hai", "String lena matlab heap pe fazool kharcha").
   - Every module's Hinglish entry must cover these standardized components:
     1. **Overview: Big Picture** (Kyun chahiye ye component, system context me kahan fit hota hai).
     2. **Goal of this Step** (Exact deliverables: structs, methods, traits, tests).
     3. **Domain Analogy & Engineering Concept** (Relatable ledger/cataloguing/clearinghouse intuition).
     4. **Data Model** (Core struct/enum code snippet; ASCII diagrams remain in English only).
     5. **Plain Thought Translation** (Dimaag me implementation thought process).
     6. **Skeleton TODO Guide** (Har TODO ka meaning aur implementation guidelines).
     7. **Andar Ki Baat (Rust Decisions)** (Why this & why not that trade-offs).
   - Every Hinglish explanation provided must be stored verbatim in `hinglish-docs.md` (no omitting, no paraphrasing, per Rule 20).
   - English teaching sections must ALWAYS be provided in parallel — never skip or substitute English with Hinglish.

3. **Retroactive Coverage**:
   - Maintain a running chronicle in `hinglish-docs.md` starting from Module 1.1 through all future modules without gaps.

