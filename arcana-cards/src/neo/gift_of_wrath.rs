//! Gift of Wrath — `{3}{R}` enchantment — Aura.
//! "Enchant artifact or creature. As long as enchanted permanent is a
//!  creature, it gets +2/+2 and has menace. When this Aura leaves the
//!  battlefield, create a 2/2 red Spirit creature token with menace."
//!
//! Partial: the +2/+2 and menace are installed (the "as long as enchanted
//! permanent is a creature" gate is approximated as an unconditional grant —
//! no attached conditional builder). The leaves-the-battlefield Spirit-token
//! payoff is GAPped — there is no Aura-leaves trigger builder available.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Gift of Wrath");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        // NOTE: artifact-or-creature widened to any permanent.
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Permanent(ObjectFilter::permanent()))
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
    // NOTE: "as long as enchanted permanent is a creature" approximated as an
    // unconditional grant — no attached conditional builder.
    // GAP: "when this Aura leaves the battlefield, create a 2/2 red Spirit
    // with menace" — no Aura-leaves trigger builder.
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_pt(
                trig.source,
                2,
                2,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Menace,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
