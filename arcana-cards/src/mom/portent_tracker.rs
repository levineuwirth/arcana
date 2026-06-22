//! Portent Tracker — `{1}{G}` 1/1 Satyr Scout.
//! `{T}: Untap target land.`
//! `{T}: Choose target battle. If an opponent protects it, remove a defense
//! counter from it. Otherwise, put a defense counter on it. Activate only as a
//! sorcery.`

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Portent Tracker");
    let satyr = reg.interner_mut().intern("Satyr");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(satyr);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Untap target land.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: untap_land,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Choose target battle. If an opponent protects it, remove a defense counter from it. Otherwise, put a defense counter on it. Activate only as a sorcery.".into(),
                cost: ActivationCost::tap_only(),
                // GAP (target): "target battle" — TargetFilter has no Battle variant,
                // so the target cannot be declared.
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: battle_defense_counter,
            }),
    )
}

fn untap_land(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Untap { target: *id }]
}

fn battle_defense_counter(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: targets a Battle (no Battle target filter), and the branch turns on
    // "if an opponent protects it" — there is no condition primitive for battle
    // protection — so neither the target nor the conditional add/remove of a
    // defense counter is expressible.
    Vec::new()
}
