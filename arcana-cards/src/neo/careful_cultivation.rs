//! Careful Cultivation — `{2}{G}` enchantment — Aura.
//! "Enchant artifact or creature. As long as enchanted permanent is a creature,
//!  it gets +1/+3 and has reach and '{T}: Add {G}{G}.' Channel — {1}{G},
//!  Discard this card: Create a 1/1 green Human Monk creature token with
//!  '{T}: Add {G}.'"
//!
//! Buff Aura on any permanent (artifact-or-creature widened to any permanent):
//! ETB installs `attached_pt(+1/+3)`, `attached_keyword(Reach)`, and a
//! host-activated `{T}: Add {G}{G}` mana ability, all following
//! `source.attached_to`. The "as long as enchanted permanent is a creature"
//! conditional gate is approximated (the grants simply ride the host), and the
//! Channel discard-from-hand alternative is not in the usable surface — both
//! are GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Careful Cultivation");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
                effect: etb_install_grants,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_grants(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "as long as enchanted permanent is a creature" conditional gate
    // approximated (grants ride the host); "Channel — {1}{G}, Discard this card"
    // discard-from-hand alternative not in the usable surface.
    let mana_ability = ActivatedAbilityDef {
        text: "{T}: Add {G}{G}.".into(),
        cost: ActivationCost {
            mana_cost: ManaCost::parse("{0}").expect("valid cost"),
            tap: true,
            ..ActivationCost::default()
        },
        target_requirements: Vec::new(),
        is_mana_ability: true,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect: add_gg,
    };
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_pt(
                trig.source,
                1,
                3,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Reach,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_activated(
                trig.source,
                mana_ability,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}

fn add_gg(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Green, ctx.source),
            ManaUnit::plain(ManaColor::Green, ctx.source),
        ],
    }]
}
