//! Battlefield entry that can pause before the move for an optional life
//! payment (A1.5bj). The request owns the entry modifiers; a continuation
//! owns the rest of the instruction, including internal multi-entry loops.

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

#[derive(Clone, Debug)]
pub(crate) struct PendingEntry {
    request: BattlefieldEntry,
    payments: std::collections::VecDeque<u32>,
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
    /// containing the optional payment are needed by generic entry routes;
    /// played lands and permanent spells supply their full lists directly.
    pub(crate) fn cache_entry_specs(&mut self, registry: &CardRegistry) {
        let cards: Vec<CardId> = self.objects.iter().map(|o| o.card_id).collect();
        for card in cards {
            self.entry_specs.entry(card).or_insert_with(|| {
                registry
                    .get(card)
                    .map(|d| &d.enters_with)
                    .filter(|specs| {
                        specs
                            .iter()
                            .any(|s| matches!(s, EntersWithSpec::TappedUnlessPayLife { .. }))
                    })
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
        let payments = request
            .clauses
            .as_ref()
            .unwrap()
            .iter()
            .filter_map(|s| match s {
                EntersWithSpec::TappedUnlessPayLife { life } => Some(*life),
                _ => None,
            })
            .collect::<std::collections::VecDeque<_>>();
        if payments.is_empty() {
            return self.finish_battlefield_entry(request);
        }
        self.pending_entry = Some(PendingEntry {
            request,
            payments,
            remaining: Default::default(),
            ready: Vec::new(),
        });
        self.prompt_entry_payment();
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
            let payments = request
                .clauses
                .as_ref()
                .unwrap()
                .iter()
                .filter_map(|s| match s {
                    EntersWithSpec::TappedUnlessPayLife { life } => Some(*life),
                    _ => None,
                })
                .collect::<std::collections::VecDeque<_>>();
            if !payments.is_empty() {
                self.pending_entry = Some(PendingEntry {
                    request,
                    payments,
                    remaining,
                    ready,
                });
                self.prompt_entry_payment();
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

    fn prompt_entry_payment(&mut self) {
        let entry = self.pending_entry.as_ref().expect("entry continuation");
        let player = entry.request.controller;
        let object_id = entry.request.object_id;
        let life = *entry.payments.front().expect("unanswered entry payment");
        self.push_pending_choice(
            player,
            ChoiceContext::EnteringBattlefield { object_id },
            ChoiceKind::OptionalCost {
                cost: OptionalPaymentKind::Life(life),
            },
        );
    }

    pub(crate) fn answer_entry_payment(&mut self, object_id: ObjectId, pay: bool) {
        let mut entry = self
            .pending_entry
            .take()
            .expect("entry payment without entry");
        assert_eq!(
            entry.request.object_id, object_id,
            "entry payment for another object"
        );
        let amount = entry.payments.pop_front().expect("entry payment amount");
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
        if entry.payments.is_empty() {
            entry.ready.push(entry.request);
            self.stage_entry_batch(entry.remaining, entry.ready);
        } else {
            self.pending_entry = Some(entry);
            self.prompt_entry_payment();
        }
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
