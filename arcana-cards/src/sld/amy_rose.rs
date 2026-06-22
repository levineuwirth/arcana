//! Amy Rose — `{2}{R}{W}` 3/3 Legendary Hedgehog Warrior with Haste.
//! "Whenever Amy Rose attacks, attach up to one target Equipment to her.
//! Then up to one other target attacking creature gets +X/+0 until end of
//! turn, where X is Amy Rose's power."
//!
//! Haste is wired. The attack trigger declares two up-to-one targets (an
//! Equipment, and another attacking creature); it attaches the chosen
//! Equipment to Amy Rose and pumps the chosen attacker by Amy Rose's
//! current power.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Amy Rose");
    let hedgehog = reg.interner_mut().intern("Hedgehog");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hedgehog);
    subtypes.0.insert(warrior);

    let equipment_filter = script::subtype_filter(reg, "Equipment");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_attach_and_pump,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![
                // up to one target Equipment
                TargetRequirement {
                    filter: TargetFilter::Permanent(equipment_filter),
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
                // up to one other target attacking creature
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().attacking_only(),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
            ],
        }),
    )
}

fn attack_attach_and_pump(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    let x = script::power_of(state, trig.source).max(0) as i32;
    let mut idx = 0;
    // First target requirement: the Equipment to attach to Amy Rose.
    if let Some(TargetChoice::Object(eq)) = trig.targets.targets.get(idx) {
        effects.push(Effect::Attach {
            equipment_or_aura: *eq,
            target: trig.source,
        });
        idx += 1;
    }
    // Second target requirement: the attacking creature to pump.
    if let Some(TargetChoice::Object(creature)) = trig.targets.targets.get(idx) {
        effects.push(Effect::Pump {
            target: *creature,
            power: x,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        });
    }
    effects
}
