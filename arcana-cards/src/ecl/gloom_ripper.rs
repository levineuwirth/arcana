//! Gloom Ripper — `{3}{B}{B}` 4/4 black Elf Assassin.
//! "When this creature enters, target creature you control gets +X/+0 until end of turn
//! and up to one target creature an opponent controls gets -0/-X until end of turn,
//! where X is the number of Elves you control plus the number of Elf cards in your graveyard."
//!
//! GAP: graveyard count (Elf cards in graveyard) not accessible; also two-target shape
//! (friendly + optional enemy) is not expressible as a simple target pair. Using
//! single-target pump on a friendly creature with just the battlefield Elf count.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gloom Ripper");
    let elf = reg.interner_mut().intern("Elf");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(assassin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

fn on_etb(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: graveyard Elf count not accessible; using battlefield Elf count only.
    let x = script::count_matching(
        state,
        &script::subtype_filter(reg, "Elf").controlled_by(ControllerConstraint::You),
        trig.controller,
    ) as i32;
    vec![Effect::Pump {
        target: *id,
        power: x,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
