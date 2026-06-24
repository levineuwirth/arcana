//! Plaguebearer — `{1}{B}` 1/1 black Zombie.
//! "{X}{X}{B}: Destroy target nonblack creature with mana value X."
//!
//! The `{X}{X}{B}` cost fans out per affordable X; the resolver reads the
//! paid X from `ctx.x_value` and destroys the chosen creature only if it is
//! a nonblack creature whose mana value equals X (the "nonblack ... with
//! mana value X" restriction, validated at resolution).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Plaguebearer");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}{X}{B}: Destroy target nonblack creature with mana value X.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}{X}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: destroy_creature,
            }),
    )
}

fn destroy_creature(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = ctx.x_value.unwrap_or(0);
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // "nonblack creature with mana value X" — validate the chosen target
    // at resolution against a nonblack-creature, exact-CMC-X filter.
    let filter = ObjectFilter::creature()
        .without_colors(ColorSet::black())
        .with_exact_cmc(x);
    let Some(obj) = state.objects.get(*id) else { return Vec::new(); };
    if !filter.matches(obj, state, ctx.controller) {
        return Vec::new();
    }
    vec![Effect::DestroyPermanent { target: *id }]
}
