//! Battlefield entry that can pause before the move for a decision its
//! controller makes as the permanent enters: an optional life payment
//! (A1.5bj) or the reveal of a card from hand. The request owns the entry
//! modifiers; a continuation owns the rest of the instruction, including
//! internal multi-entry loops.

use crate::actions::{ChoiceContext, ChoiceKind, OptionalPaymentKind};
use crate::effects::{DigRest, Effect};
use crate::events::{GameEvent, MoveCause};
use crate::objects::{Characteristics, ObjectId};
use crate::registry::{CardRegistry, EntersWithSpec};
use crate::state::GameState;
use crate::types::{CardId, ColorSet, CounterKind, PlayerId};
use crate::zones::Zone;

#[derive(Clone, Debug)]
pub(crate) struct BattlefieldEntry {
    pub object_id: ObjectId,
    pub controller: PlayerId,
    pub cause: MoveCause,
    pub clauses: Option<Vec<EntersWithSpec>>,
    pub x_value: Option<u32>,
    pub delve_count: u32,
    pub tapped: bool,
    pub attacking: bool,
    pub face_down: bool,
    pub counters: Vec<(CounterKind, u32)>,
    pub colors_spent: Option<ColorSet>,
    pub attached_to: Option<ObjectId>,
    pub was_cast: bool,
    pub resolved_spell: Option<ObjectId>,
    pub reveal: bool,
    /// The entering permanent's timestamp, drawn ahead for permanents that
    /// enter together (CR 613.7m). `None` draws one as the entry finishes.
    pub timestamp: Option<u64>,
}

impl BattlefieldEntry {
    pub fn new(object_id: ObjectId, controller: PlayerId, cause: MoveCause) -> Self {
        Self {
            object_id,
            controller,
            cause,
            clauses: None,
            x_value: None,
            delve_count: 0,
            tapped: false,
            attacking: false,
            face_down: false,
            counters: Vec::new(),
            colors_spent: None,
            attached_to: None,
            was_cast: false,
            resolved_spell: None,
            reveal: false,
            timestamp: None,
        }
    }
}

/// One decision an entering permanent's controller makes before it moves,
/// in the order the card's clauses list them (CR 614.12).
#[derive(Clone, Debug)]
pub(crate) enum EntryDecision {
    /// "You may pay N life. If you don't, it enters tapped."
    PayLife(u32),
    /// "You may reveal a [filter] card from your hand. If you don't, it
    /// enters tapped." Boxed: a filter is far larger than a life amount.
    Reveal(Box<crate::targets::ObjectFilter>),
}

impl EntryDecision {
    /// Whether a declaration needs an answer before the move.
    fn is_asked_by(spec: &EntersWithSpec) -> bool {
        matches!(
            spec,
            EntersWithSpec::TappedUnlessPayLife { .. } | EntersWithSpec::TappedUnlessReveal { .. }
        )
    }

    /// The decisions a clause list asks for, in the list's order.
    fn asked_by(clauses: &[EntersWithSpec]) -> std::collections::VecDeque<EntryDecision> {
        clauses
            .iter()
            .filter_map(|s| match s {
                EntersWithSpec::TappedUnlessPayLife { life } => Some(EntryDecision::PayLife(*life)),
                EntersWithSpec::TappedUnlessReveal { filter } => {
                    Some(EntryDecision::Reveal(Box::new(filter.clone())))
                }
                _ => None,
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct PendingEntry {
    request: BattlefieldEntry,
    decisions: std::collections::VecDeque<EntryDecision>,
    remaining: std::collections::VecDeque<BattlefieldEntry>,
    ready: Vec<BattlefieldEntry>,
}

#[derive(Clone, Debug)]
pub(crate) enum EntryContinuation {
    Enter(BattlefieldEntry),
    EnterBatch(Vec<BattlefieldEntry>),
    Effect(Box<Effect>),
    Move {
        id: ObjectId,
        destination: Zone,
        cause: MoveCause,
    },
    DigRest {
        player: PlayerId,
        ids: Vec<ObjectId>,
        rest: DigRest,
    },
    LibraryOrder {
        player: PlayerId,
        cards: Vec<ObjectId>,
        placements: Vec<(ObjectId, crate::actions::CardDestination)>,
    },
}

impl GameState {
    /// The zone mover has no registry argument. Cache the declarations of
    /// cards present in this game at the engine boundary. Only declarations
    /// that ask for a decision are needed by generic entry routes; played
    /// lands and permanent spells supply their full lists directly.
    pub(crate) fn cache_entry_specs(&mut self, registry: &CardRegistry) {
        let cards: Vec<CardId> = self.objects.iter().map(|o| o.card_id).collect();
        for card in cards {
            self.entry_specs.entry(card).or_insert_with(|| {
                registry
                    .get(card)
                    .map(|d| &d.enters_with)
                    .filter(|specs| specs.iter().any(EntryDecision::is_asked_by))
                    .cloned()
                    .unwrap_or_default()
            });
        }
    }

    /// A selected face-up card is known to its incoming controller even
    /// while it waits in a hidden zone. Otherwise sampled worlds can give
    /// this same id a different identity before its entry choice is answered.
    fn remember_entering_card(&mut self, request: &BattlefieldEntry) {
        if !request.face_down
            && self
                .objects
                .get(request.object_id)
                .is_some_and(|o| o.zone.is_hidden())
        {
            if let Some(player) = self.players.get_mut(request.controller as usize) {
                player.known_cards.insert(request.object_id);
            }
        }
    }

    /// Returns the new id for an immediate entry; None if entry is parked
    /// or the object is absent. All post-entry modifications must be carried
    /// by the request, rather than depending on an immediate return value.
    pub(crate) fn enter_battlefield(&mut self, mut request: BattlefieldEntry) -> Option<ObjectId> {
        self.remember_entering_card(&request);
        if self.pending_choice.is_some() {
            self.entry_continuations
                .push_back(EntryContinuation::Enter(request));
            return None;
        }
        let obj = self.objects.get(request.object_id)?;
        if obj.zone == Zone::Battlefield {
            return None;
        }
        if request.clauses.is_none() {
            request.clauses = Some(
                self.entry_specs
                    .get(&obj.card_id)
                    .cloned()
                    .unwrap_or_default(),
            );
        }
        // A manifested card has no printed abilities as it enters.
        if request.face_down {
            request.clauses = Some(Vec::new());
        }
        let decisions = EntryDecision::asked_by(request.clauses.as_ref().unwrap());
        if decisions.is_empty() {
            return self.finish_battlefield_entry(request);
        }
        self.pending_entry = Some(PendingEntry {
            request,
            decisions,
            remaining: Default::default(),
            ready: Vec::new(),
        });
        self.prompt_entry_decision();
        None
    }

    /// All replacement choices for a batch precede every entry in that
    /// batch. Payments are charged in order, so later choices cannot spend
    /// life already committed to an earlier entry (CR 614.12b).
    pub(crate) fn enter_battlefield_batch(&mut self, requests: Vec<BattlefieldEntry>) {
        // Preserve the whole selection, including ready cards and later
        // payments, before the first choice can expose it to sampling.
        for request in &requests {
            self.remember_entering_card(request);
        }
        if self.pending_choice.is_some() {
            self.entry_continuations
                .push_back(EntryContinuation::EnterBatch(requests));
        } else {
            self.stage_entry_batch(requests.into(), Vec::new());
        }
    }

    fn stage_entry_batch(
        &mut self,
        mut remaining: std::collections::VecDeque<BattlefieldEntry>,
        mut ready: Vec<BattlefieldEntry>,
    ) {
        while let Some(mut request) = remaining.pop_front() {
            let Some(obj) = self.objects.get(request.object_id) else {
                continue;
            };
            if obj.zone == Zone::Battlefield {
                continue;
            }
            if request.clauses.is_none() {
                request.clauses = Some(
                    self.entry_specs
                        .get(&obj.card_id)
                        .cloned()
                        .unwrap_or_default(),
                );
            }
            if request.face_down {
                request.clauses = Some(Vec::new());
            }
            let decisions = EntryDecision::asked_by(request.clauses.as_ref().unwrap());
            if !decisions.is_empty() {
                self.pending_entry = Some(PendingEntry {
                    request,
                    decisions,
                    remaining,
                    ready,
                });
                self.prompt_entry_decision();
                return;
            }
            ready.push(request);
        }
        // CR 613.7m: permanents that enter together get their timestamps
        // in APNAP order, the active player's first and then each other
        // player's in turn order, in the batch's own order within a player
        // (A1.5br). Only the timestamps follow that order; the entries are
        // still finished in the order the instruction listed them.
        if ready.len() > 1 {
            let active = self.active_player() as usize;
            let players = self.num_players() as usize;
            let seat = |r: &BattlefieldEntry|
                (r.controller as usize + players - active) % players;
            let mut order: Vec<usize> = (0..ready.len()).collect();
            order.sort_by_key(|&i| seat(&ready[i]));
            for i in order {
                ready[i].timestamp = Some(self.next_timestamp());
            }
        }
        for request in ready {
            self.finish_battlefield_entry(request);
        }
    }

    /// Ask the entering permanent's controller the decision at the front
    /// of its queue.
    fn prompt_entry_decision(&mut self) {
        let entry = self.pending_entry.as_ref().expect("entry continuation");
        let player = entry.request.controller;
        let object_id = entry.request.object_id;
        let kind = match entry.decisions.front().expect("unanswered entry decision") {
            EntryDecision::PayLife(life) => ChoiceKind::OptionalCost {
                cost: OptionalPaymentKind::Life(*life),
            },
            // Up to one of the cards that can be revealed; picking none
            // declines. With no such card the choice holds only the decline.
            EntryDecision::Reveal(filter) => ChoiceKind::PickCards {
                candidates: self.entry_reveal_candidates(player, object_id, filter),
                min: 0,
                max: 1,
            },
        };
        self.push_pending_choice(
            player,
            ChoiceContext::EnteringBattlefield { object_id },
            kind,
        );
    }

    /// The cards `player` may reveal for an entering permanent's reveal
    /// clause: the cards in hand that match `filter` by their
    /// characteristics there, never the entering card itself. One option
    /// per distinct card, since revealing either of two Plains is the same
    /// act, taking the lowest object id of each; in object-id order.
    fn entry_reveal_candidates(
        &self,
        player: PlayerId,
        entering: ObjectId,
        filter: &crate::targets::ObjectFilter,
    ) -> Vec<ObjectId> {
        let mut matching: Vec<(ObjectId, CardId)> = self
            .objects
            .objects_in_zone(Zone::Hand(player))
            .filter(|o| o.id != entering && filter.matches(o, self, player))
            .map(|o| (o.id, o.card_id))
            .collect();
        matching.sort_unstable();
        let mut seen: Vec<CardId> = Vec::new();
        matching
            .into_iter()
            .filter(|(_, card)| {
                let fresh = !seen.contains(card);
                if fresh {
                    seen.push(*card);
                }
                fresh
            })
            .map(|(id, _)| id)
            .collect()
    }

    /// The reveal clause the pending entry is waiting on, for the view's
    /// prompt. `None` when no entry waits on a reveal.
    pub(crate) fn pending_entry_reveal_filter(&self) -> Option<&crate::targets::ObjectFilter> {
        match self.pending_entry.as_ref()?.decisions.front()? {
            EntryDecision::Reveal(filter) => Some(filter.as_ref()),
            EntryDecision::PayLife(_) => None,
        }
    }

    fn take_pending_entry(&mut self, object_id: ObjectId) -> PendingEntry {
        let entry = self
            .pending_entry
            .take()
            .expect("entry decision without entry");
        assert_eq!(
            entry.request.object_id, object_id,
            "entry decision for another object"
        );
        entry
    }

    /// The decision at the front of the queue is answered: ask the next
    /// one, or let the batch go on once this entry has none left.
    fn continue_pending_entry(&mut self, mut entry: PendingEntry) {
        if entry.decisions.is_empty() {
            entry.ready.push(entry.request);
            self.stage_entry_batch(entry.remaining, entry.ready);
        } else {
            self.pending_entry = Some(entry);
            self.prompt_entry_decision();
        }
    }

    pub(crate) fn answer_entry_payment(&mut self, object_id: ObjectId, pay: bool) {
        let mut entry = self.take_pending_entry(object_id);
        let Some(EntryDecision::PayLife(amount)) = entry.decisions.pop_front() else {
            panic!("entry payment answered, but the entry asked for something else");
        };
        if pay {
            let life = self.player(entry.request.controller).life;
            assert!(
                amount == 0 || (life >= 0 && life as u32 >= amount),
                "unaffordable entry payment"
            );
            crate::effects::lose_life(self, entry.request.controller, amount);
        } else {
            // Paying another clause must never undo this, or another
            // independent tapped requirement.
            entry.request.tapped = true;
        }
        self.continue_pending_entry(entry);
    }

    /// Answer an entry's reveal clause with the card picked, or with none.
    /// A revealed card becomes known to every player and stays in hand;
    /// declining taps the entering permanent, which no other clause's
    /// answer undoes.
    pub(crate) fn answer_entry_reveal(&mut self, object_id: ObjectId, picked: &[ObjectId]) {
        let mut entry = self.take_pending_entry(object_id);
        let Some(EntryDecision::Reveal(_)) = entry.decisions.pop_front() else {
            panic!("entry reveal answered, but the entry asked for something else");
        };
        // The pick is one of the candidates the prompt listed; the
        // dispatcher has checked that. The card is not matched against the
        // filter again: in a world another player sampled while this prompt
        // waited, the listed id can hold a different card, and the prompt's
        // list is what the choice was made from.
        match picked {
            [] => entry.request.tapped = true,
            [card] => {
                for p in 0..self.num_players() {
                    self.player_mut(p).known_cards.insert(*card);
                }
            }
            _ => panic!("entry reveal of more than one card"),
        }
        self.continue_pending_entry(entry);
    }

    fn finish_battlefield_entry(&mut self, request: BattlefieldEntry) -> Option<ObjectId> {
        let id = request.object_id;
        let from = self.objects.get(id)?.zone;
        if from == Zone::Battlefield {
            return None;
        }
        // A non-resolution effect can move a spell directly off the stack.
        if from == Zone::Stack {
            if let Some(pos) = self.stack.iter().position(|e| e.id == id && e.is_spell()) {
                let entry = self.stack.remove(pos);
                if let Some(obj) = self.objects.get_mut(id) {
                    if let Some(chars) = entry.pre_adventure_characteristics {
                        obj.characteristics = chars;
                    }
                    if let Some(chars) = entry.pre_split_characteristics {
                        obj.characteristics = chars;
                        obj.visible_face = 0;
                    }
                }
            }
        }
        let (new_id, from) = self.swap_to_zone_reid(id, Zone::Battlefield)?;
        let obj = self.objects.get_mut(new_id).unwrap();
        obj.controller = request.controller;
        if let Some(colors) = request.colors_spent {
            obj.colors_paid = colors;
        }
        if request.face_down {
            obj.status.face_down = true;
            obj.characteristics = Characteristics {
                power: Some(crate::types::PtValue::Fixed(2)),
                toughness: Some(crate::types::PtValue::Fixed(2)),
                types: crate::types::TypeLine::new().with(crate::types::TypeLine::CREATURE),
                ..Default::default()
            };
        }
        self.emit(GameEvent::ZoneChange {
            object_id: id,
            from,
            to: Zone::Battlefield,
            new_id,
            cause: request.cause,
        });
        crate::stack::apply_enters_with_clauses(
            self,
            new_id,
            request.clauses.as_deref().unwrap_or(&[]),
            request.x_value,
            request.delve_count,
        );
        for (kind, count) in request.counters {
            self.place_counters(
                crate::replacement::CounterTarget::Object(new_id),
                kind,
                count,
            );
        }
        self.after_enter_battlefield_at(new_id, request.timestamp);
        if request.tapped {
            self.objects.get_mut(new_id).unwrap().tap();
        }
        if request.attacking {
            crate::effects::enter_tapped_attacking(self, new_id, request.controller);
        }
        if let Some(target) = request.attached_to {
            Effect::Attach {
                equipment_or_aura: new_id,
                target,
            }
            .execute(self);
        }
        self.emit(GameEvent::EntersBattlefield {
            object_id: new_id,
            from_zone: from,
            was_cast: request.was_cast,
        });
        if request.reveal {
            for p in 0..self.num_players() {
                self.player_mut(p).known_cards.insert(new_id);
            }
        }
        if let Some(object_id) = request.resolved_spell {
            self.emit(GameEvent::SpellResolved { object_id });
        }
        Some(new_id)
    }

    pub(crate) fn drain_entry_continuations(&mut self) {
        let previous_resolving = self.currently_resolving;
        if let Some(parked) = &self.pending_resolution {
            self.currently_resolving = Some(parked.entry.id);
        }
        while self.pending_choice.is_none() && !self.entry_continuations.is_empty() {
            // New work generated by the current instruction precedes the
            // outer tail. This matters for nested Sequence/ForEach bodies.
            let mut tail = std::mem::take(&mut self.entry_continuations);
            match tail.pop_front().unwrap() {
                EntryContinuation::Enter(request) => {
                    self.enter_battlefield(request);
                }
                EntryContinuation::EnterBatch(requests) => self.enter_battlefield_batch(requests),
                EntryContinuation::Effect(effect) => effect.execute(self),
                EntryContinuation::Move {
                    id,
                    destination,
                    cause,
                } => {
                    self.move_object_to_zone(id, destination, cause);
                }
                EntryContinuation::DigRest { player, ids, rest } => {
                    crate::effects::apply_dig_rest(self, player, &ids, rest)
                }
                EntryContinuation::LibraryOrder {
                    player,
                    cards,
                    placements,
                } => crate::engine::finish_library_order(self, player, &cards, &placements),
            }
            self.entry_continuations.extend(tail);
        }
        self.currently_resolving = previous_resolving;
    }
}
