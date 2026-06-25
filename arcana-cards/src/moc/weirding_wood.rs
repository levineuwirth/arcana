//! Weirding Wood — `{2}{G}` enchantment — Aura.
//! "Enchant land. When this Aura enters, investigate. Enchanted land has
//!  \"{T}: Add two mana of any one color.\""
//!
//! Enchant land. ETB investigates (`CreateCommodityToken { Clue }`) and installs
//! the host mana ability. "{T}: Add two mana of any one color" is modeled as five
//! granted activated abilities (one per WUBRG color, each adding TWO mana of that
//! color), installed on the host land via `attached_activated` (Squirrel Nest
//! idiom); the shared {T} cost on the host means only one fires.

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Weirding Wood");
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
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Permanent(
                ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
            ))
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
    let grant = |text: &str, effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>| {
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_activated(
                trig.source,
                ActivatedAbilityDef {
                    text: text.into(),
                    cost: ActivationCost::tap_only(),
                    target_requirements: Vec::new(),
                    is_mana_ability: true,
                    is_loyalty_ability: false,
                    activation_zone: ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect,
                },
                Duration::WhileSourceOnBattlefield,
            ),
        }
    };
    vec![
        Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Clue,
            count: 1,
        },
        grant("{T}: Add {W}{W}.", add_white_mana),
        grant("{T}: Add {U}{U}.", add_blue_mana),
        grant("{T}: Add {B}{B}.", add_black_mana),
        grant("{T}: Add {R}{R}.", add_red_mana),
        grant("{T}: Add {G}{G}.", add_green_mana),
    ]
}

fn add_white_mana(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::White, ctx.source),
            ManaUnit::plain(ManaColor::White, ctx.source),
        ],
    }]
}

fn add_blue_mana(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Blue, ctx.source),
            ManaUnit::plain(ManaColor::Blue, ctx.source),
        ],
    }]
}

fn add_black_mana(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Black, ctx.source),
            ManaUnit::plain(ManaColor::Black, ctx.source),
        ],
    }]
}

fn add_red_mana(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Red, ctx.source),
            ManaUnit::plain(ManaColor::Red, ctx.source),
        ],
    }]
}

fn add_green_mana(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Green, ctx.source),
            ManaUnit::plain(ManaColor::Green, ctx.source),
        ],
    }]
}
