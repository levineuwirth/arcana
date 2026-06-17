//! June, Bounty Hunter — `{1}{B}` 2/2 Legendary Human Mercenary.
//! "June can't be blocked as long as you've drawn two or more cards this turn.
//!  (conditional static — GAP'd)
//!  {1}, Sacrifice another creature: Create a Clue token. Activate only during
//!  your turn."

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("June, Bounty Hunter");
    let human = reg.interner_mut().intern("Human");
    let mercenary = reg.interner_mut().intern("Mercenary");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(mercenary);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "can't be blocked as long as you've drawn two or more cards this
    // turn" is a conditional evasion static — not a triggered/activated ability.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Sacrifice another creature: Create a Clue token. Activate only during your turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                // GAP: "Activate only during your turn" timing restriction has
                // no exposed activation-condition helper in this catalog.
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_clue,
            }),
    )
}

fn make_clue(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: ctx.controller,
        kind: CommodityToken::Clue,
        count: 1,
    }]
}
