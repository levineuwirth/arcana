//! Pendelhaven — Legendary Land (Legends, 1994).
//! "{T}: Add {G}." and "{T}: Target 1/1 creature gets +1/+2 until end of
//! turn." The 1/1 restriction is approximated with power == 1 and
//! toughness <= 1 (ObjectFilter has no minimum-toughness refinement —
//! GAP noted; a legal on-battlefield creature with toughness < 1 is dead
//! anyway, so the approximation is tight).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pendelhaven");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {G}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_green_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Target 1/1 creature gets +1/+2 until end of \
                       turn."
                    .into(),
                cost: ActivationCost::tap_only(),
                // GAP: no minimum-toughness refinement — "1/1" is
                // approximated as power exactly 1 and toughness at most 1.
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .with_min_power(1)
                            .with_max_power(1)
                            .with_max_toughness(1),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump,
            }),
    )
}

fn add_green_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}

fn pump(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 1,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
