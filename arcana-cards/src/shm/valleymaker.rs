//! Valleymaker — `{5}{R/G}` 5/5 Giant Shaman (red/green).
//!
//! * `{T}, Sacrifice a Mountain: This creature deals 3 damage to target creature.`
//! * `{T}, Sacrifice a Forest: Choose a player. That player adds {G}{G}{G}.`
//!
//! Two activated abilities. Each has a tap cost plus a chosen-permanent
//! sacrifice (`sacrifice_other`) filtered to the basic land subtype.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let mountain_filter = script::subtype_filter(reg, "Mountain");
    let forest_filter = script::subtype_filter(reg, "Forest");

    let name = reg.interner_mut().intern("Valleymaker");
    let giant = reg.interner_mut().intern("Giant");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R/G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice a Mountain: This creature deals 3 damage to target creature.".into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice_other: Some(mountain_filter),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_three,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice a Forest: Choose a player. That player adds {G}{G}{G}.".into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice_other: Some(forest_filter),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: choose_player_add_ggg,
            }),
    )
}

fn deal_three(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount: 3,
    }]
}

fn choose_player_add_ggg(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ChoosePlayerThen {
        chooser: ctx.controller,
        opponents_only: false,
        then: Box::new(Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source); 3],
        }),
    }]
}
