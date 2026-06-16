//! Unable to Scream — `{U}` enchantment — Aura.
//! "Enchant creature. Enchanted creature loses all abilities and is a Toy
//!  artifact creature with base power and toughness 0/2 in addition to its
//!  other types. As long as enchanted creature is face down, it can't be
//!  turned face up."
//!
//! ETB sets the host's base P/T to 0/2 (`attached_set_pt`), adds the artifact
//! card type (`attached_types`), and adds the Toy creature subtype
//! (`attached_subtypes`). The "loses all abilities" clause has no attached
//! ability-removal builder in this surface, and the face-down "can't be turned
//! face up" clause has no primitive — both GAP'd.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unable to Scream");
    let aura = reg.interner_mut().intern("Aura");
    let _toy = reg.interner_mut().intern("Toy");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
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
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "loses all abilities" — no attached ability-removal builder here.
    // GAP: "as long as enchanted creature is face down, it can't be turned
    //      face up" — no face-down restriction primitive.
    let mut toy_set = SubtypeSet::default();
    if let Some(toy) = reg.interner().lookup("Toy") {
        toy_set.0.insert(toy);
    }
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_set_pt(
                trig.source,
                0,
                2,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_types(
                trig.source,
                TypeLine::ARTIFACT.into(),
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_subtypes(
                trig.source,
                toy_set,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
