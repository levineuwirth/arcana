//! The Master, Formed Anew — `{U}{B}` 0/1 Legendary Time Lord Rogue.
//!
//! * "Body Thief — When you cast this spell, you may exile a creature
//!   you control and put a takeover counter on it." — modeled as a
//!   cast trigger, but the chosen-creature exile-plus-named-counter
//!   payload feeds the enter-as-copy mechanic below and has no faithful
//!   primitive chain (exiling a chosen creature you control and tagging
//!   the exiled card with a takeover counter), so the effect is GAP'd.
//! * "You may have The Master enter as a copy of a creature card in
//!   exile with a takeover counter on it." — a static enter-as-copy
//!   replacement with no engine primitive; GAP'd. Body Thief is not a
//!   KeywordAbility variant.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Master, Formed Anew");
    let time_lord = reg.interner_mut().intern("Time Lord");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(time_lord);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: static "You may have The Master enter as a copy of a creature card
    // in exile with a takeover counter on it." — enter-as-copy replacement,
    // no engine primitive. Body Thief is not a KeywordAbility variant.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: body_thief_exile,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn body_thief_exile(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may exile a creature you control and put a takeover counter
    // on it" — exiling a chosen creature you control and tagging the exiled
    // card with a takeover counter (to feed the enter-as-copy mechanic) has
    // no faithful primitive chain.
    Vec::new()
}
