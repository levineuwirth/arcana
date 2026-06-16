//! Mightstone's Animation — `{3}{U}` enchantment — Aura.
//! "Enchant artifact. When this Aura enters, draw a card. Enchanted
//!  artifact is a creature with base power and toughness 4/4 in addition
//!  to its other types."
//!
//! ETB draws a card. The animation is an ETB-installed `attached_set_pt`
//! (base 4/4) plus `attached_types` adding the Creature type.

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
    let name = reg.interner_mut().intern("Mightstone's Animation");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
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
}

fn etb_animate(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    if let Some(you) = state.object_or_lki(trig.source).map(|o| o.controller) {
        effects.push(Effect::DrawCards {
            player: you,
            count: 1,
        });
    }
    effects.push(Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_set_pt(
            trig.source,
            4,
            4,
            Duration::WhileSourceOnBattlefield,
        ),
    });
    effects.push(Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_types(
            trig.source,
            TypeLine::CREATURE.into(),
            Duration::WhileSourceOnBattlefield,
        ),
    });
    effects
}
