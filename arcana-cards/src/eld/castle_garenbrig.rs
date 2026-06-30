//! Castle Garenbrig — ELD utility land. "This land enters tapped unless you
//! control a Forest" (`EntersWithSpec::TappedUnlessControl`), "{T}: Add {G}",
//! and "{2}{G}{G}, {T}: Add six {G}. Spend this mana only to cast creature
//! spells or activate abilities of creatures." Modeled with
//! `SpendRestriction::OnlyCastCreatureSpells` (the "activate abilities of
//! creatures" half is elided — closest available restriction).

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaRestrictions, ManaUnit, SpendRestriction};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Castle Garenbrig");
    let forest = reg.interner_mut().intern("Forest");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::TappedUnlessControl {
                filter: ObjectFilter::permanent().with_subtype_sym(forest),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {G}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_green,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{G}{G}, {T}: Add six {G}. Spend this mana only to cast creature spells or activate abilities of creatures.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{G}{G}").expect("valid cost"),
                    ..ActivationCost::tap_only()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ramp_six_green,
            }),
    )
}

fn add_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}

fn ramp_six_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mana: Vec<ManaUnit> = (0..6)
        .map(|_| ManaUnit {
            color: ManaColor::Green,
            source: ctx.source,
            restrictions: ManaRestrictions {
                spend_only_on: Some(SpendRestriction::OnlyCastCreatureSpells),
                is_snow: false,
            },
        })
        .collect();
    vec![Effect::AddMana { player: ctx.controller, mana }]
}
