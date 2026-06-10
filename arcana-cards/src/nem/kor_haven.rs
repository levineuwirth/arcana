//! Kor Haven — legendary land (Nemesis, 2000).
//! "{T}: Add {C}." and "{1}{W}, {T}: Prevent all combat damage that
//! would be dealt by target attacking creature this turn."
//! The prevention activation is wired (cost + target) but its effect is
//! a GAP: prevention of damage dealt BY one specific object is not
//! expressible (`Effect::PreventDamage` prevents damage TO a target;
//! `Effect::PreventDamageFrom` is filter-based, not id-based).

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, ManaColor, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kor Haven");
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
                text: "{1}{W}, {T}: Prevent all combat damage that would \
                       be dealt by target attacking creature this turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{W}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                // GAP: "target ATTACKING creature" — no attacking-status
                // refinement on ObjectFilter in this catalog; plain
                // target creature declared instead.
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_combat_damage_by_target,
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

fn prevent_combat_damage_by_target(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "prevent all combat damage that would be dealt BY target
    // creature" — PreventDamage only prevents damage TO a target and
    // PreventDamageFrom takes an ObjectFilter (no per-id source form),
    // so the by-this-object prevention cannot be expressed.
    Vec::new()
}
