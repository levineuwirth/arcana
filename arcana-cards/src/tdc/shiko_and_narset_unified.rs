//! Shiko and Narset, Unified — `{1}{U}{R}{W}` Legendary 4/4 Human Spirit Dragon.
//!
//! Oracle:
//! * Flying, vigilance.
//! * Flurry — Whenever you cast your second spell each turn, copy that spell if
//!   it targets a permanent or player, and you may choose new targets for the
//!   copy. If you don't copy a spell this way, draw a card.
//!
//! Flurry is an ability word (not a usable KeywordAbility). The trigger is
//! emitted, gated to the second spell each turn via an intervening-if on the
//! turn's spell count. Its payload is GAP'd: there is no PendingTrigger accessor
//! for the triggering spell's stack object, so "copy THAT spell" cannot target,
//! and emitting only the "otherwise draw a card" fallback would be a materially
//! wrong card (it would always draw).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shiko and Narset, Unified");
    let human = reg.interner_mut().intern("Human");
    let spirit = reg.interner_mut().intern("Spirit");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(spirit);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: Some(if_second_spell),
                effect: flurry,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_second_spell(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    // "your second spell each turn" — the just-cast spell is already counted
    // when the trigger goes on the stack, so the second spell is count == 2.
    script::spells_cast_this_turn(s, &ObjectFilter::new(), you) == 2
}

fn flurry(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "copy that spell if it targets a permanent or player, ... If you
    // don't copy a spell this way, draw a card." There is no accessor for the
    // triggering spell's stack object, so "copy that spell" cannot target.
    // Emitting only the draw fallback would be materially wrong, so the whole
    // payload is GAP'd.
    Vec::new()
}
