//! Sivriss, Nightmare Speaker — `{3}{B}` 3/3 Legendary Snake Cleric Warlock.
//! "{T}, Sacrifice another creature or an artifact: For each opponent, you mill a
//! card, then return that card from your graveyard to your hand unless that player
//! pays 3 life. Choose a Background."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sivriss, Nightmare Speaker");
    let snake = reg.interner_mut().intern("Snake");
    let cleric = reg.interner_mut().intern("Cleric");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(cleric);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: "Choose a Background" (commander variant rule) has no KeywordAbility.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Sacrifice another creature or an artifact: For each opponent, you mill a card, then return that card from your graveyard to your hand unless that player pays 3 life.".into(),
            cost: ActivationCost {
                tap: true,
                sacrifice_other: Some(
                    ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::ARTIFACT)),
                ),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: mill_per_opponent,
        }),
    )
}

fn mill_per_opponent(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // Faithful: you mill one card for each opponent.
    // GAP: "then return that milled card to your hand unless that player pays 3
    // life" — requires tracking the specific milled card per opponent and a
    // per-opponent unless-pay gate, which is not expressible here.
    let opps = script::opponents(state, ctx.controller);
    opps.into_iter()
        .map(|_| Effect::Mill {
            player: ctx.controller,
            count: 1,
        })
        .collect()
}
