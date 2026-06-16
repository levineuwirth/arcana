//! Zoetic Glyph — `{2}{U}` enchantment — Aura.
//! "Enchant artifact. Enchanted artifact is a Golem creature with base
//!  power and toughness 5/4 in addition to its other types. When this
//!  Aura is put into a graveyard from the battlefield, discover 3."
//!
//! ETB installs the animation: `attached_set_pt(5/4)` +
//! `attached_types(CREATURE)` + `attached_subtypes(Golem)`. The
//! self-leaves "discover 3" trigger is GAP'd — a payoff that fires when
//! the Aura itself is put into a graveyard is not expressible here.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zoetic Glyph");
    let aura = reg.interner_mut().intern("Aura");
    // Interned here so the ETB effect fn can `lookup` it.
    let _golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Permanent(
                ObjectFilter::permanent().with_types(TypeLine::ARTIFACT.into()),
            ))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_animate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
    // GAP: "When this Aura is put into a graveyard from the battlefield,
    // discover 3" — self-leaves payoff not expressible.
}

fn etb_animate(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let golem = reg.interner().lookup("Golem").expect("Golem interned");
    let mut s = SubtypeSet::default();
    s.0.insert(golem);
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_set_pt(
                trig.source,
                5,
                4,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_types(
                trig.source,
                TypeLine::CREATURE.into(),
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_subtypes(
                trig.source,
                s,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
