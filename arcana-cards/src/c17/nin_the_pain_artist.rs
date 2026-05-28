//! Nin, the Pain Artist — `{U}{R}` 1/1 blue/red Legendary Vedalken Wizard.
//! "{X}{U}{R}, {T}: Nin deals X damage to target creature. That creature's controller
//! draws X cards."
//! GAP: X-cost mana + draw-X-cards where X = damage is expressible if ctx.x_value works.
//! Using ctx.x_value to read X.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nin, the Pain Artist");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}{U}{R}, {T}: Nin deals X damage to target creature. That creature's controller draws X cards.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}{R}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_x_draw_x,
            }),
    )
}

fn deal_x_draw_x(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let x = ctx.x_value.unwrap_or(0);
    if x == 0 { return Vec::new(); }
    let controller = script::target_controller(state, *id, ctx.controller);
    vec![
        Effect::DealDamage {
            target: DamageTarget::Object(*id),
            amount: x,
            source: ctx.source,
        },
        Effect::DrawCards { player: controller, count: x },
    ]
}
