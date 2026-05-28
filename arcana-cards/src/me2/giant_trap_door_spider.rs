//! Giant Trap Door Spider — `{1}{R}{G}` 2/3 red/green Spider.
//! "{1}{R}{G}, {T}: Exile this creature and target creature without flying that's
//! attacking you."
//! GAP: Targeting "a creature without flying that's attacking you" requires a combat
//! state filter (attacking + no flying + targeting you) not expressible in ObjectFilter.
//! Also exiling this creature as part of the effect (not as a cost) is unusual.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Giant Trap Door Spider");
    let spider = reg.interner_mut().intern("Spider");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}{G}, {T}: Exile this creature and target creature without flying that's attacking you.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}{G}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: trap_door_exile,
            }),
    )
}

fn trap_door_exile(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: The filter "without flying, attacking you" is not expressible. The exile
    // of this creature itself as an effect is emitted via ExilePermanent on ctx.source.
    vec![
        Effect::ExilePermanent { target: *id },
        Effect::ExilePermanent { target: ctx.source },
    ]
}
