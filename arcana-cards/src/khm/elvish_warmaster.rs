//! Elvish Warmaster — `{1}{G}` 2/2 green Elf Warrior.
//!
//! * "Whenever one or more other Elves you control enter, create a 1/1
//!   green Elf Warrior creature token. This ability triggers only once
//!   each turn." — a battlefield ZoneChange watching Elves you control,
//!   capped to `TriggerFrequency::OncePerTurn`, minting a 1/1 green Elf
//!   Warrior token.
//! * "{5}{G}{G}: Elves you control get +2/+2 and gain deathtouch until
//!   end of turn." — a mana-cost activated ability pumping every Elf you
//!   control (ForEach over the matching ids), granting deathtouch via
//!   the Pump keyword rider.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elvish Warmaster");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);

    let elf_filter = script::subtype_filter(reg, "Elf").controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: elf_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: make_elf_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{G}{G}: Elves you control get +2/+2 and gain deathtouch until end of turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{G}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_elves,
            }),
    )
}

fn make_elf_token(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let elf = reg.interner().lookup("Elf").unwrap_or_default();
    let warrior = reg.interner().lookup("Warrior").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: elf,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn pump_elves(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Elf").controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, ctx.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: 2,
            toughness: 2,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Deathtouch],
        }),
    }]
}
