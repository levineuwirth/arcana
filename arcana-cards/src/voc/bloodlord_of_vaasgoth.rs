//! Bloodlord of Vaasgoth — `{3}{B}{B}` 3/3 Vampire Warrior.
//!
//! Oracle:
//! * Bloodthirst 3 (parametrized keyword — enters with three +1/+1 counters if
//!   an opponent was dealt damage this turn).
//! * Flying.
//! * "Whenever you cast a Vampire creature spell, it gains bloodthirst 3." —
//!   grants a keyword to a spell still on the stack; there is no Effect to add a
//!   keyword ability to a spell object (GrantKeyword targets a permanent with a
//!   Duration). The trigger is registered; the effect is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloodlord of Vaasgoth");
    let vampire = reg.interner_mut().intern("Vampire");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Bloodthirst(3), KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::creature().with_subtype_sym(vampire),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: grant_bloodthirst,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn grant_bloodthirst(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "it gains bloodthirst 3" — no Effect grants a keyword ability to a
    // spell on the stack (GrantKeyword targets a permanent with a Duration).
    Vec::new()
}
