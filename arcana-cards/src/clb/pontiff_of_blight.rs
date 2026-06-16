//! Pontiff of Blight — `{4}{B}{B}` 2/7 Zombie Cleric.
//! "Extort (Whenever you cast a spell, you may pay {W/B}. If you do,
//! each opponent loses 1 life and you gain that much life.)"
//! "Other creatures you control have extort." (static grant)

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pontiff of Blight");
    let zombie = reg.interner_mut().intern("Zombie");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(7)),
        // Extort is not an expressible KeywordAbility variant; modeled as the
        // cast-trigger below.
        ..Default::default()
    };

    // GAP: static "Other creatures you control have extort" — granting an
    // ability to other permanents is a continuous static, not expressible here.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: extort,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn extort(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opponents = script::opponents(state, trig.controller);
    let gained = opponents.len() as u32;
    let mut inner: Vec<Effect> = opponents
        .into_iter()
        .map(|p| Effect::LoseLife { player: p, amount: 1 })
        .collect();
    if gained > 0 {
        inner.push(Effect::GainLife {
            player: trig.controller,
            amount: gained,
        });
    }
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{W/B}").expect("valid cost")),
        then: Box::new(Effect::Sequence(inner)),
        else_effect: None,
    }]
}
