//! Parasitic Implant — `{3}{B}` enchantment — Aura.
//! "Enchant creature. At the beginning of your upkeep, enchanted
//! creature's controller sacrifices it and you create a 1/1 colorless
//! Phyrexian Myr artifact creature token."
//!
//! id:1 — ETB boilerplate (no static grant on the host).
//! id:2 — at the beginning of YOUR (the Aura controller's) upkeep:
//! create the 1/1 colorless Phyrexian Myr artifact-creature token. The
//! "enchanted creature's controller sacrifices it" half is a GAP — there
//! is no sacrifice-a-specific-object effect (only DestroyPermanent,
//! which is destruction, not sacrifice).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Parasitic Implant");
    let aura = reg.interner_mut().intern("Aura");
    let _phyrexian = reg.interner_mut().intern("Phyrexian");
    let _myr = reg.interner_mut().intern("Myr");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_noop,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_make_myr,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
    // GAP: "enchanted creature's controller sacrifices it" — no
    // sacrifice-a-specific-object effect (only DestroyPermanent exists,
    // which is destruction rather than sacrifice).
}

fn etb_noop(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}

fn upkeep_make_myr(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let phyrexian = reg.interner().lookup("Phyrexian").expect("Phyrexian interned");
    let myr = reg.interner().lookup("Myr").expect("Myr interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(myr);
    let token = TokenDefinition {
        name: myr,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
