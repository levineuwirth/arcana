//! Intruding Soulrager — `{U}{R}` 2/2 Spirit (red/blue) with Vigilance.
//! "{T}, Sacrifice a Room: This creature deals 2 damage to each opponent.
//!  Draw a card."
//!
//! Vigilance is a base keyword. The activation taps and sacrifices a
//! chosen Room (`sacrifice_other`), then deals 2 to each opponent and
//! draws a card.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Intruding Soulrager");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![arcana_core::effects::KeywordAbility::Vigilance],
        ..Default::default()
    };

    let room_filter = script::subtype_filter(reg, "Room");

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Sacrifice a Room: This creature deals 2 damage to each \
                   opponent. Draw a card."
                .into(),
            cost: ActivationCost {
                tap: true,
                sacrifice_other: Some(room_filter),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: damage_opponents_and_draw,
        }),
    )
}

fn damage_opponents_and_draw(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = script::opponents(state, ctx.controller)
        .into_iter()
        .map(|p| Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(p),
            amount: 2,
        })
        .collect();
    effects.push(Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    });
    vec![Effect::Sequence(effects)]
}
