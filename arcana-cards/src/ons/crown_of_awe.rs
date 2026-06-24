//! Crown of Awe — `{1}{W}` enchantment — Aura.
//! "Enchant creature. Enchanted creature has protection from black and
//!  from red. Sacrifice this Aura: Enchanted creature and other creatures
//!  that share a creature type with it gain protection from black and from
//!  red until end of turn."
//!
//! The ETB trigger installs the static grant: two `attached_keyword`
//! Protection effects — `ProtectionQuality::Color(Black)` and
//! `Color(Red)` — following `source.attached_to` while this Aura remains
//! on the battlefield (one install per protected quality).
//! GAP: the "Sacrifice this Aura: tribal protection until end of turn"
//! activated ability needs a sacrifice-cost, shared-creature-type sweep
//! that isn't expressible here.

use arcana_core::effects::{Effect, KeywordAbility, ProtectionQuality};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, Color, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crown of Awe");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
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
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Sacrifice this Aura: tribal protection until end of turn" — the
    // shared-creature-type sweep with a sacrifice cost isn't expressible.
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Protection(ProtectionQuality::Color(Color::Black)),
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Protection(ProtectionQuality::Color(Color::Red)),
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
