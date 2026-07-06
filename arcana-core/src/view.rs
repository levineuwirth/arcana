//! Serializable view of a game for UI frontends — the keystone for the
//! web/server GUI (the live [`GameState`]/`Session` can't serialize because it
//! holds fn-pointer-bearing continuous/replacement/triggered effects, so we send
//! a flattened projection instead) and equally usable by a native egui app
//! reading it in-process.
//!
//! [`view_state`] projects `(GameState, perspective, legal actions)` into a
//! [`ViewState`] of plain serde data: life, zone counts, the perspective
//! player's hand, every battlefield permanent (name + computed P/T + tapped),
//! the stack, and the legal actions with human-readable labels. `name` is enough
//! for a frontend to fetch card art (e.g. from Scryfall). Hidden information is
//! respected — only the perspective player's hand contents are filled in; other
//! hands are counts only. Pass a perspective-projected state if you also want
//! opponents' hidden cards anonymized at the source.

use serde::{Deserialize, Serialize};

use crate::actions::Action;
use crate::objects::ObjectId;
use crate::registry::CardRegistry;
use crate::render::render_action;
use crate::state::{GameResult, GameState};
use crate::types::PlayerId;
use crate::zones::Zone;

/// One battlefield/hand/stack object, flattened for display.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CardView {
    pub id: ObjectId,
    /// Card name — also the key a frontend uses to fetch art (Scryfall, etc.).
    /// Empty for a hidden/anonymized object.
    pub name: String,
    /// Rendered mana cost ("{2}{G}{G}"); `None` for objects with no cost (lands,
    /// tokens) or hidden objects.
    pub mana_cost: Option<String>,
    /// Mana value (converted mana cost) — 0 if no cost.
    pub mana_value: u32,
    /// Printed type line ("Legendary Creature — Elf Warrior", "Basic Land —
    /// Forest", "Instant"). Empty for a hidden object.
    pub type_line: String,
    /// Whether this object is a land — lets a frontend tally untapped mana
    /// sources without re-parsing the type line.
    pub is_land: bool,
    /// For the perspective player's hand only: true if this card could be cast
    /// or played this turn assuming the player taps out (see
    /// [`crate::legal_actions::playable_cards`]). Always false for opponents'
    /// objects, the battlefield, and the stack.
    pub playable: bool,
    /// For the perspective player's BATTLEFIELD permanents only: true if it has an
    /// activated ability the player could activate this turn assuming they tap out
    /// for mana (potential, mana-floated). Lets a card-driven UI offer activation
    /// (auto-tapping the cost) even when the mana isn't floated yet. False
    /// elsewhere.
    #[serde(default)]
    pub activatable: bool,
    /// Printed keyword abilities by stable glossary key ("Flying", "Ward", …),
    /// deduped + sorted. A frontend pairs these with
    /// [`crate::glossary::keyword_glossary`] for reminder text.
    pub keywords: Vec<String>,
    /// Computed power/toughness; `None` for non-creatures.
    pub power: Option<i32>,
    pub toughness: Option<i32>,
    pub tapped: bool,
}

/// A mana amount broken down by color — used for the "available mana" gauge.
/// `total == white + blue + black + red + green + colorless`.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ManaCounts {
    pub white: usize,
    pub blue: usize,
    pub black: usize,
    pub red: usize,
    pub green: usize,
    pub colorless: usize,
    pub total: usize,
}

fn mana_counts(pool: &crate::mana::ManaPool) -> ManaCounts {
    use crate::types::ManaColor;
    let mut c = ManaCounts::default();
    for u in pool.iter() {
        match u.color {
            ManaColor::White => c.white += 1,
            ManaColor::Blue => c.blue += 1,
            ManaColor::Black => c.black += 1,
            ManaColor::Red => c.red += 1,
            ManaColor::Green => c.green += 1,
            ManaColor::Colorless => c.colorless += 1,
        }
        c.total += 1;
    }
    c
}

/// One player's public state plus (for the perspective player) their hand.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PlayerView {
    pub id: PlayerId,
    pub life: i32,
    pub hand_count: usize,
    pub library_count: usize,
    pub graveyard_count: usize,
    /// Cards in this player's exile (owner-filtered; Exile is a shared zone).
    pub exile_count: usize,
    /// Floating (unspent) mana in this player's pool — usually 0 between
    /// decisions, non-zero mid-cast.
    pub mana_pool: usize,
    /// This player's energy counters ({E}) — public. Shown in the HUD when > 0.
    pub energy: u32,
    /// This player's poison counters — public. 10 = loss (CR 104.3c). Shown in
    /// the HUD when > 0 (Toxic / Infect).
    pub poison: u32,
    /// Mana this player could produce right now (floating pool + everything
    /// their untapped mana abilities can still make), by color. Drives the
    /// "available mana" gauge. See [`crate::legal_actions::available_mana`] for
    /// the flexible-source caveat.
    pub available_mana: ManaCounts,
    /// Filled only for the perspective player (hidden information).
    pub hand: Vec<CardView>,
    pub battlefield: Vec<CardView>,
    /// This player's graveyard contents (public — both players' are visible), for
    /// the zone viewer. Order is the graveyard's iteration order.
    pub graveyard: Vec<CardView>,
    /// This player's exiled cards (public exile; face-down exile would need its
    /// own treatment, not modeled here). For the zone viewer.
    pub exile: Vec<CardView>,
}

/// A legal action plus its display label and stable index into the `legal`
/// slice the frontend was given (the frontend sends the index back to apply).
///
/// `source` is the game object the action acts FROM when there is one — the land
/// played, the spell cast, or the permanent whose ability is activated — so a
/// card-driven UI can map a click on that card to its legal action(s) instead of
/// rendering a button. `None` for actions with no single source (pass, choices,
/// combat declarations).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ActionView {
    pub index: usize,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<ObjectId>,
    /// The action's chosen target(s), if any — so a card-driven UI can offer
    /// board-click targeting (highlight these, click one to apply) and draw a
    /// source→target arrow. Empty for untargeted actions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<TargetRef>,
}

/// A single target of an action, for the targeting UI: an object (permanent /
/// card on the stack) or a player.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TargetRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<ObjectId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub player: Option<PlayerId>,
}

/// The object an action acts FROM, for card-driven UIs (see [`ActionView::source`]).
fn action_source(a: &Action) -> Option<ObjectId> {
    match a {
        Action::PlayLand { object_id, .. } => Some(*object_id),
        Action::CastSpell { object_id, .. } => Some(*object_id),
        Action::ActivateAbility { source, .. } => Some(*source),
        _ => None,
    }
}

/// The action's chosen targets, for board-click targeting (see [`ActionView::targets`]).
fn action_targets(a: &Action) -> Vec<TargetRef> {
    let sel = match a {
        Action::CastSpell { targets, .. } => Some(targets),
        Action::ActivateAbility { targets, .. } => Some(targets),
        _ => None,
    };
    sel.map(|s| s.targets.iter().map(|t| TargetRef {
        object: t.object_id(),
        player: t.player_id(),
    }).collect()).unwrap_or_default()
}

/// A complete, serializable snapshot of the game from one player's view.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ViewState {
    pub turn: u32,
    /// "{phase:?}/{step:?}".
    pub phase: String,
    pub active_player: PlayerId,
    pub priority_player: PlayerId,
    /// Whose view this is.
    pub perspective: PlayerId,
    pub players: Vec<PlayerView>,
    pub stack: Vec<CardView>,
    pub legal: Vec<ActionView>,
    /// A pending card-pick the perspective player must make (a library search /
    /// tutor / reanimation), surfaced as a visual picker. `None` otherwise.
    pub choice: Option<ChoiceView>,
    /// A one-line description of the decision the perspective player faces when
    /// it isn't a visual picker (e.g. "Choose a target for Guide of Souls"),
    /// shown as a header above the action buttons so it's clear what they're
    /// answering. `None` when there's no special pending decision.
    pub prompt: Option<String>,
    /// Set once the game is decided ("Win(0)" / "Draw" / …).
    pub game_over: Option<String>,
}

/// One pickable card in a [`ChoiceView`], paired with the `legal` index that
/// selects it (so the frontend submits via the normal action path).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChoiceCardOption {
    pub card: CardView,
    pub action: usize,
}

/// A pending "pick a card from a zone" choice (PickCards) rendered for a visual
/// picker: the prompt, the pickable candidates (the applicable cards), the full
/// searched zone with the candidates FIRST (so you see your whole library with
/// the matches at the front), and the `legal` index of the "pick nothing"
/// option when the choice is optional.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChoiceView {
    pub prompt: String,
    pub min: u32,
    pub max: u32,
    pub options: Vec<ChoiceCardOption>,
    /// Every card in the searched zone (candidates first), for visualization.
    pub pool: Vec<CardView>,
    /// `legal` index of "pick nothing" (present when min == 0).
    pub decline: Option<usize>,
}

/// Build the visual-picker [`ChoiceView`] for a pending PickCards the
/// perspective player owns. `legal` is the same action slice the frontend
/// indexes, so each option carries the index that selects it.
///
/// Pass the *authoritative* (un-anonymized) state: a library lives in a hidden
/// zone, so an information-set projection blanks the very cards a search needs
/// to reveal. The web layer recomputes this from `Session::state()` for that
/// reason.
///
/// Scoped to SEARCHABLE card zones (library / graveyard / exile). Battlefield
/// and hand picks (sacrifice, discard, enlist, …) stay as on-board / text
/// choices — a full-screen card grid there would hijack their existing UI.
pub fn build_choice_view(
    state: &GameState, registry: &CardRegistry, perspective: PlayerId, legal: &[Action],
) -> Option<ChoiceView> {
    use crate::actions::{ChoiceFollowUp, ChoiceKind, ChoiceResponse};
    use crate::zones::Zone;
    let pc = state.pending_choice.as_ref()?;
    if pc.choosing_player != perspective {
        return None;
    }
    let ChoiceKind::PickCards { candidates, min, max } = &pc.kind else { return None; };

    // The searched zone is wherever the candidates live; only show the visual
    // picker for zones a player searches/browses through a grid.
    let zone = match candidates.first().and_then(|&id| state.objects.get(id)).map(|o| o.zone) {
        Some(z @ (Zone::Library(_) | Zone::Graveyard(_) | Zone::Exile)) => z,
        _ => return None,
    };

    let pick_index = |id: ObjectId| legal.iter().position(|a| matches!(a,
        Action::SubmitResolutionChoice { response: ChoiceResponse::PickCards { picked }, .. }
            if picked.len() == 1 && picked[0] == id));
    let decline = legal.iter().position(|a| matches!(a,
        Action::SubmitResolutionChoice { response: ChoiceResponse::PickCards { picked }, .. }
            if picked.is_empty()));

    let options: Vec<ChoiceCardOption> = candidates.iter()
        .filter_map(|&id| pick_index(id).map(|action| ChoiceCardOption {
            card: card_view(state, registry, id), action,
        }))
        .collect();

    // Pool = the searched zone, matching candidates first. A true search lets
    // you look at the WHOLE zone (CR 701.19); but a "look at the top N" effect
    // (DigTopN) only reveals the cards looked at — don't leak the rest of the
    // library, so its pool stays the candidates alone.
    let cand: std::collections::HashSet<ObjectId> = candidates.iter().copied().collect();
    let mut pool: Vec<CardView> = candidates.iter()
        .map(|&id| card_view(state, registry, id)).collect();
    let look_at_whole_zone = !matches!(
        state.pending_choice_follow_up.as_ref(),
        Some(ChoiceFollowUp::DigTopFinish { .. }));
    if look_at_whole_zone {
        for o in state.objects.objects_in_zone(zone) {
            if !cand.contains(&o.id) { pool.push(card_view(state, registry, o.id)); }
        }
    }
    let prompt = match zone {
        Zone::Library(_) => "Search your library",
        Zone::Graveyard(_) => "Choose a card from the graveyard",
        Zone::Exile => "Choose an exiled card",
        _ => "Choose a card",
    }.to_string();
    Some(ChoiceView { prompt, min: *min, max: *max, options, pool, decline })
}

/// A one-line description of the pending decision for `perspective`, so the UI
/// can show "what am I answering?" above the action buttons. Returns `None` for
/// PickCards (the visual picker carries its own prompt) and when there's no
/// special pending decision.
fn build_decision_prompt(
    state: &GameState, registry: &CardRegistry, perspective: PlayerId,
) -> Option<String> {
    use crate::actions::ChoiceKind;
    // Special actions carry no `pending_choice`, so they'd otherwise show bare
    // "Choose <card>" buttons with no context. Name the cleanup discard.
    if state.priority.special_action == Some(crate::priority::SpecialAction::DiscardToHandSize)
        && state.priority.player == perspective
    {
        let max = state.effective_max_hand_size(perspective);
        let over = state.objects.count_in_zone(Zone::Hand(perspective))
            .saturating_sub(max);
        return Some(format!(
            "Discard {over} card{} — you're over the {max}-card limit",
            if over == 1 { "" } else { "s" },
        ));
    }
    let pc = state.pending_choice.as_ref()?;
    if pc.choosing_player != perspective {
        return None;
    }
    let name_of = |id: ObjectId| -> Option<String> {
        let o = state.objects.get(id)?;
        registry.interner().resolve(o.characteristics.name)
            .filter(|s| !s.is_empty()).map(|s| s.to_string())
    };
    match &pc.kind {
        // "Target X" buttons are otherwise context-free — name the source whose
        // ability is asking (the stack entry's source permanent).
        ChoiceKind::ChooseTargets { source } => {
            let who = state.stack.iter().find(|e| e.id == *source)
                .and_then(|e| name_of(e.source));
            Some(match who {
                Some(n) => format!("Choose a target for {n}"),
                None => "Choose a target".to_string(),
            })
        }
        ChoiceKind::PickPlayer { .. } => Some("Choose a player".to_string()),
        ChoiceKind::ChooseColor => Some("Choose a color".to_string()),
        // Scry / surveil / fateseal — name the effect so the ordering buttons
        // ("Top: … | Bottom: …") read in context.
        ChoiceKind::OrderCards { cards, allowed } => {
            use crate::actions::CardDestination;
            let verb = if allowed.contains(&CardDestination::Graveyard) {
                "Surveil"
            } else if allowed.contains(&CardDestination::BottomOfLibrary) {
                "Scry"
            } else {
                "Order"
            };
            Some(format!("{verb} {} — choose where each card goes", cards.len()))
        }
        // PickCards is shown by the visual picker (its own prompt); others have
        // self-explanatory buttons (Yes/No, order, optional cost).
        _ => None,
    }
}

fn card_view(state: &GameState, registry: &CardRegistry, id: ObjectId) -> CardView {
    let Some(o) = state.objects.get(id) else {
        return CardView {
            id, name: String::new(), mana_cost: None, mana_value: 0,
            type_line: String::new(), is_land: false, playable: false, activatable: false,
            keywords: Vec::new(), power: None, toughness: None, tapped: false,
        };
    };
    let c = &o.characteristics;
    let mut name = registry.interner().resolve(c.name)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_default();
    let mana_cost = c.mana_cost.as_ref().map(|mc| mc.to_string());
    let mana_value = c.mana_value();
    let mut type_line = crate::catalog::card_type_line(c, registry);
    // Commodity tokens (Treasure / Clue / Food / …) are minted without an
    // interned name; render their kind so they don't show up blank.
    if let Some(kind) = o.commodity {
        if name.is_empty() { name = kind.display_name().to_string(); }
        if !type_line.contains('—') {
            type_line = format!("{type_line} — {}", kind.display_name());
        }
    }
    let is_land = c.types.is_land();
    let (power, toughness) = if c.types.is_creature() {
        (state.computed_power(id), state.computed_toughness(id))
    } else {
        (None, None)
    };
    let tapped = o.is_tapped();
    let mut keywords: Vec<String> = c.keywords.iter()
        .map(crate::glossary::keyword_key).collect();
    keywords.sort_unstable();
    keywords.dedup();
    CardView {
        id, name, mana_cost, mana_value, type_line, is_land,
        playable: false, activatable: false, keywords, power, toughness, tapped,
    }
}

/// Project `state` (from `perspective`'s view) plus its `legal` actions into a
/// [`ViewState`]. See module docs for the hidden-information contract.
pub fn view_state(
    state: &GameState,
    registry: &CardRegistry,
    perspective: PlayerId,
    legal: &[Action],
) -> ViewState {
    let cards_in = |zone: Zone| -> Vec<CardView> {
        let ids: Vec<ObjectId> = state.objects.objects_in_zone(zone).map(|o| o.id).collect();
        ids.into_iter().map(|id| card_view(state, registry, id)).collect()
    };
    // Exile is a single shared zone — a player's exiled cards are those they own.
    let exile_of = |p: PlayerId| -> Vec<CardView> {
        let ids: Vec<ObjectId> = state.objects.objects_in_zone(Zone::Exile)
            .filter(|o| o.owner == p).map(|o| o.id).collect();
        ids.into_iter().map(|id| card_view(state, registry, id)).collect()
    };

    // Which of the perspective player's hand cards are playable this turn (cast
    // or play if they tap out). Computed once; only that player's hand is shown.
    let playable: std::collections::HashSet<ObjectId> =
        crate::legal_actions::playable_cards(state, perspective, registry)
            .into_iter().collect();

    // Permanents the perspective player could ACTIVATE this turn if they tapped
    // out (potential, mana-floated) — sources of activate actions in the
    // mana-floated enumeration. Lets the UI offer auto-tap activation (Codie).
    let activatable: std::collections::HashSet<ObjectId> =
        crate::legal_actions::activatable_sources(state, perspective, registry);

    let players = (0..state.num_players()).map(|p| {
        PlayerView {
            id: p,
            life: state.player(p).life,
            hand_count: state.objects.objects_in_zone(Zone::Hand(p)).count(),
            library_count: state.objects.objects_in_zone(Zone::Library(p)).count(),
            graveyard_count: state.objects.objects_in_zone(Zone::Graveyard(p)).count(),
            exile_count: state.objects.objects_in_zone(Zone::Exile)
                .filter(|o| o.owner == p).count(),
            mana_pool: state.player(p).mana_pool.total(),
            energy: state.player(p).energy,
            poison: state.player(p).poison_counters,
            available_mana: mana_counts(&crate::legal_actions::available_mana(state, p, registry)),
            hand: if p == perspective {
                let mut h = cards_in(Zone::Hand(p));
                for c in &mut h { c.playable = playable.contains(&c.id); }
                h
            } else {
                Vec::new()
            },
            battlefield: {
                let ids: Vec<ObjectId> = state.objects.objects_in_zone(Zone::Battlefield)
                    .filter(|o| o.controller == p).map(|o| o.id).collect();
                ids.into_iter().map(|id| {
                    let mut c = card_view(state, registry, id);
                    if p == perspective { c.activatable = activatable.contains(&id); }
                    c
                }).collect()
            },
            graveyard: cards_in(Zone::Graveyard(p)),
            exile: {
                let mut ex = exile_of(p);
                // Impulse-exiled cards you "may play" surface as castable, so the
                // UI can highlight + offer them (cast-from-exile). Only your own.
                if p == perspective {
                    for c in &mut ex { c.playable = playable.contains(&c.id); }
                }
                ex
            },
        }
    }).collect();

    let stack = state.stack.iter()
        .filter_map(|e| e.card_id().map(|_| e.id))
        .map(|id| card_view(state, registry, id))
        .collect();

    // Build the visual-picker choice from the raw action slice (its indices
    // match the frontend's) BEFORE projecting `legal` into ActionViews.
    let choice = build_choice_view(state, registry, perspective, legal);
    let prompt = build_decision_prompt(state, registry, perspective);

    let legal = legal.iter().enumerate()
        .map(|(index, a)| ActionView {
            index,
            label: render_action(a, state, registry),
            source: action_source(a),
            targets: action_targets(a),
        })
        .collect();

    let game_over = match state.result {
        Some(GameResult::Win(p)) => Some(format!("Win(P{p})")),
        Some(GameResult::Draw) => Some("Draw".to_string()),
        Some(GameResult::Eliminated(p)) => Some(format!("Eliminated(P{p})")),
        None => None,
    };

    ViewState {
        turn: state.turn.turn_number,
        phase: format!("{:?}/{:?}", state.turn.phase, state.turn.step),
        active_player: state.active_player(),
        priority_player: state.priority_player(),
        perspective,
        players,
        stack,
        legal,
        choice,
        prompt,
        game_over,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::{ChoiceContext, ChoiceKind, ChoiceResponse};
    use crate::objects::{Characteristics, GameObject};

    /// A pending `PickCards` over a subset of the library projects into
    /// `ViewState.choice` as a visual search: the matching candidates become
    /// clickable options (each carrying a real legal action index), and the
    /// pool exposes the whole searched zone with candidates listed first.
    fn seed_library(s: &mut GameState, count: usize) -> Vec<ObjectId> {
        let mut ids = Vec::new();
        for _ in 0..count {
            let id = s.allocate_object_id();
            s.objects.insert(GameObject::new(
                id, 0, Zone::Library(0), 1, Characteristics::default()));
            ids.push(id);
        }
        s.player_mut(0).library_top_to_bottom = ids.clone();
        ids
    }

    #[test]
    fn pick_cards_surfaces_as_visual_search_choice() {
        let mut s = GameState::new(2, 0);
        let lib = seed_library(&mut s, 5);
        // Search finds the first two of the five library cards.
        let candidates = vec![lib[0], lib[1]];
        s.push_pending_choice(
            0, ChoiceContext::ResolvingStack(crate::objects::NULL_OBJECT_ID),
            ChoiceKind::PickCards { candidates: candidates.clone(), min: 0, max: 1 });

        let registry = CardRegistry::new();
        let legal = crate::legal_actions::legal_actions(&s, &registry);
        let view = view_state(&s, &registry, 0, &legal);

        let ch = view.choice.expect("pending PickCards should surface as a choice");
        // One option per candidate, each pointing at a real legal action that
        // picks exactly that id.
        assert_eq!(ch.options.len(), candidates.len());
        for opt in &ch.options {
            assert!(candidates.contains(&opt.card.id));
            match &legal[opt.action] {
                Action::SubmitResolutionChoice {
                    response: ChoiceResponse::PickCards { picked }, ..
                } => assert_eq!(picked, &vec![opt.card.id]),
                other => panic!("option action is not a single-card pick: {other:?}"),
            }
        }
        // min == 0 → "pick nothing" is offered and is itself a legal action.
        let decline = ch.decline.expect("min 0 should offer a decline");
        assert!(matches!(&legal[decline],
            Action::SubmitResolutionChoice {
                response: ChoiceResponse::PickCards { picked }, .. } if picked.is_empty()));
        // Pool = the whole library, candidates first.
        assert_eq!(ch.pool.len(), lib.len());
        assert_eq!(ch.pool[0].id, lib[0]);
        assert_eq!(ch.pool[1].id, lib[1]);
    }

    #[test]
    fn battlefield_picks_do_not_open_the_search_modal() {
        // Sacrifice / destroy / enlist etc. are PickCards too, but they're
        // chosen on the board — a full-screen card grid would hijack that UI.
        let mut s = GameState::new(2, 0);
        let id = s.allocate_object_id();
        s.objects.insert(GameObject::new(
            id, 0, Zone::Battlefield, 1, Characteristics::default()));
        s.push_pending_choice(
            0, ChoiceContext::ResolvingStack(crate::objects::NULL_OBJECT_ID),
            ChoiceKind::PickCards { candidates: vec![id], min: 1, max: 1 });
        let registry = CardRegistry::new();
        let legal = crate::legal_actions::legal_actions(&s, &registry);
        assert!(view_state(&s, &registry, 0, &legal).choice.is_none());
    }

    #[test]
    fn dig_top_n_reveals_only_the_cards_looked_at() {
        // "Look at the top N" lets you see only those N — the picker must NOT
        // expand its pool to the rest of the library (information leak).
        use crate::actions::ChoiceFollowUp;
        use crate::effects::DigRest;
        let mut s = GameState::new(2, 0);
        let lib = seed_library(&mut s, 6);
        let looked = vec![lib[0], lib[1]];
        s.pending_choice_follow_up = Some(ChoiceFollowUp::DigTopFinish {
            player: 0, looked_at: looked.clone(), rest: DigRest::BottomRandom });
        s.push_pending_choice(
            0, ChoiceContext::ResolvingStack(crate::objects::NULL_OBJECT_ID),
            ChoiceKind::PickCards { candidates: looked.clone(), min: 0, max: 1 });
        let registry = CardRegistry::new();
        let legal = crate::legal_actions::legal_actions(&s, &registry);
        let ch = view_state(&s, &registry, 0, &legal).choice.expect("dig picker");
        assert_eq!(ch.pool.len(), looked.len(), "dig pool must not leak the library");
    }

    #[test]
    fn exiled_cards_show_under_their_owner() {
        // Exile is one shared zone; each player's view shows the cards THEY own
        // (so impulse-exiled cards you may play surface on your side).
        let mut s = GameState::new(2, 0);
        let mine = s.allocate_object_id();
        let theirs = s.allocate_object_id();
        s.objects.insert(GameObject::new(mine, 0, Zone::Exile, 1, Characteristics::default()));
        s.objects.insert(GameObject::new(theirs, 1, Zone::Exile, 1, Characteristics::default()));
        let reg = CardRegistry::new();
        let legal = crate::legal_actions::legal_actions(&s, &reg);
        let view = view_state(&s, &reg, 0, &legal);
        let p0: Vec<_> = view.players[0].exile.iter().map(|c| c.id).collect();
        let p1: Vec<_> = view.players[1].exile.iter().map(|c| c.id).collect();
        assert!(p0.contains(&mine) && !p0.contains(&theirs), "you see your own exile");
        assert!(p1.contains(&theirs) && !p1.contains(&mine), "owner-filtered");
        assert_eq!(view.players[0].exile_count, 1);
    }

    #[test]
    fn pending_choices_surface_a_decision_prompt() {
        // A non-picker pending decision surfaces a one-line prompt above the
        // action buttons (so e.g. "Target X" buttons aren't context-free).
        use crate::actions::{ChoiceContext, ChoiceKind};
        let reg = CardRegistry::new();

        let mut s = GameState::new(2, 0);
        s.push_pending_choice(0, ChoiceContext::Sba, ChoiceKind::ChooseColor);
        let legal = crate::legal_actions::legal_actions(&s, &reg);
        assert_eq!(view_state(&s, &reg, 0, &legal).prompt.as_deref(), Some("Choose a color"));

        let mut s = GameState::new(2, 0);
        s.push_pending_choice(0, ChoiceContext::Sba,
            ChoiceKind::PickPlayer { candidates: vec![0, 1] });
        let legal = crate::legal_actions::legal_actions(&s, &reg);
        assert_eq!(view_state(&s, &reg, 0, &legal).prompt.as_deref(), Some("Choose a player"));

        // The other seat sees no prompt (not their decision).
        assert_eq!(view_state(&s, &reg, 1, &legal).prompt, None);
    }

    #[test]
    fn cleanup_discard_surfaces_a_discard_prompt() {
        // The cleanup discard is a special action (no pending_choice); it still
        // needs a prompt so the "Choose <card>" buttons read as a discard.
        use crate::objects::{GameObject, Characteristics};
        use crate::priority::SpecialAction;
        let reg = CardRegistry::new();
        let mut s = GameState::new(2, 0);
        let max = s.effective_max_hand_size(0) as usize;
        for _ in 0..max + 2 {
            let id = s.allocate_object_id();
            s.objects.insert(GameObject::new(
                id, 0, Zone::Hand(0), 0, Characteristics::default()));
        }
        s.priority.begin_special_action(SpecialAction::DiscardToHandSize, 0);
        let legal = crate::legal_actions::legal_actions(&s, &reg);
        assert_eq!(view_state(&s, &reg, 0, &legal).prompt.as_deref(),
            Some("Discard 2 cards — you're over the 7-card limit"));
        // The non-discarding seat sees no prompt.
        assert_eq!(view_state(&s, &reg, 1, &legal).prompt, None);
    }

    #[test]
    fn commodity_token_renders_its_kind_name() {
        // A Treasure token is minted name-less (no interner at resolution); the
        // view must still show "Treasure" / "Artifact — Treasure", not blank.
        let mut s = GameState::new(2, 0);
        let id = s.allocate_object_id();
        let mut chars = Characteristics::default();
        chars.types = crate::types::TypeLine::ARTIFACT.into();
        let mut o = GameObject::new(id, 0, Zone::Battlefield, 0, chars);
        o.is_token = true;
        o.commodity = Some(crate::effects::CommodityToken::Treasure);
        s.objects.insert(o);
        let reg = CardRegistry::new();
        let legal = crate::legal_actions::legal_actions(&s, &reg);
        let view = view_state(&s, &reg, 0, &legal);
        let cv = view.players[0].battlefield.iter().find(|c| c.id == id)
            .expect("token on the battlefield");
        assert_eq!(cv.name, "Treasure");
        assert!(cv.type_line.contains("Treasure"), "type line: {}", cv.type_line);
    }

    #[test]
    fn choice_is_hidden_from_the_other_seat() {
        let mut s = GameState::new(2, 0);
        let lib = seed_library(&mut s, 3);
        s.push_pending_choice(
            0, ChoiceContext::ResolvingStack(crate::objects::NULL_OBJECT_ID),
            ChoiceKind::PickCards { candidates: vec![lib[0]], min: 1, max: 1 });
        let registry = CardRegistry::new();
        let legal = crate::legal_actions::legal_actions(&s, &registry);
        // Opponent's perspective: not their decision, so no picker.
        assert!(view_state(&s, &registry, 1, &legal).choice.is_none());
    }
}
