//! Tetsuo, Imperial Champion — `{U}{B}{R}` 3/3 Legendary Human Samurai.
//!
//! Whenever Tetsuo attacks, if it's equipped, choose one —
//! • Tetsuo deals damage equal to the greatest mana value among Equipment
//!   attached to it to any target.
//! • You may cast an instant or sorcery spell from your hand with mana value
//!   less than or equal to the greatest mana value among Equipment attached
//!   to Tetsuo without paying its mana cost.
//!
//! The attacks trigger is wired (SelfAttacks), but both modes depend on the
//! greatest mana value among attached Equipment — not computable from the
//! catalog — and the free-cast-from-hand mode has no expressible effect, so
//! the whole effect body is GAP'd. The "if it's equipped" intervening-if and
//! the modal choice are likewise GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tetsuo, Imperial Champion");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: "if it's equipped" intervening-if not expressible.
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: on_attack,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_attack(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal choice depending on greatest mana value among attached
    // Equipment (not computable), and free-cast-from-hand mode (no expressible
    // effect).
    Vec::new()
}
