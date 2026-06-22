//! Vile Redeemer — `{2}{G}` 3/3 Eldrazi with Devoid (colorless) and Flash.
//! When you cast this spell, you may pay {C}. If you do, create a 1/1 colorless
//! Eldrazi Scion creature token for each nontoken creature that died under your
//! control this turn. Those tokens have "Sacrifice this token: Add {C}."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::targets::ControllerConstraint;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vile Redeemer");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    // Devoid → the card has no color.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP (trigger fidelity): "when you cast THIS spell" has no dedicated
            // self-cast variant; SpellCast{caster:You} is the closest but over-fires.
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: cast_redeem,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn cast_redeem(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "create a 1/1 Eldrazi Scion for EACH nontoken creature that died under
    // your control this turn" — there is no script helper for the count of all
    // (untyped) nontoken creatures that died this turn (only the per-subtype
    // creatures_of_subtype_died_this_turn exists), so the dynamic token count is
    // not computable. The {C} optional cost + the Scion token's "Sacrifice: Add
    // {C}" sub-ability are likewise unexpressible without the count.
    Vec::new()
}
