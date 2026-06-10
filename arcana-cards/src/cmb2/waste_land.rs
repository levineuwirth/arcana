//! Waste Land — nonbasic land (Mystery Booster playtest / Strip Mine
//! variant). "{T}: Add {C}." and "{T}, Sacrifice Waste Land: Destroy
//! target nonbasic land. That land's controller creates a Wastes token.
//! (It's a land with {T}: Add {C}.)"
//! The destroy and the Wastes land token are wired; the token's printed
//! "{T}: Add {C}" mana ability is a documented GAP (hand-rolled
//! TokenDefinitions carry no activated abilities).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::mana::ManaUnit;
use arcana_core::types::{
    CardId, ColorSet, ManaColor, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Waste Land");
    let _wastes = reg.interner_mut().intern("Wastes");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice Waste Land: Destroy target nonbasic \
                       land. That land's controller creates a Wastes token."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine::LAND.into())
                            .without_supertypes(
                                SupertypeSet::new().with(SupertypeSet::BASIC),
                            ),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: destroy_land_make_wastes,
            }),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn destroy_land_make_wastes(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let controller = script::target_controller(state, *id, ctx.controller);
    let wastes = reg.interner().lookup("Wastes").unwrap_or_default();
    // GAP: the Wastes token's printed "{T}: Add {C}" mana ability —
    // hand-rolled TokenDefinitions carry no activated abilities.
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::CreateToken {
            controller,
            token: TokenDefinition {
                name: wastes,
                colors: ColorSet::new(),
                types: TypeLine::LAND.into(),
                subtypes: SubtypeSet::default(),
                power: None,
                toughness: None,
                keywords: vec![],
                abilities: vec![],
            },
        },
    ]
}
