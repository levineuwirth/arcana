//! The Karst, Enchanted — `{2}{U}{G}` 4/4 Legendary blue/green Enchantment Creature
//! — Elemental. "{3}{U}, {T}, Sacrifice another creature: Search your library for up
//! to four cards that don't share a mana value, power, toughness, or card type with
//! each other. Target opponent chooses two of those cards. Put the chosen cards into
//! your graveyard and the rest into your hand. Then shuffle."
//! GAP: "Search for cards that don't share mv/power/toughness/type with each other"
//! is an extremely complex search constraint not expressible in ObjectFilter.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Karst, Enchanted");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{G}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE).into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{U}, {T}, Sacrifice another creature: Search your library for up to four cards that don't share a mana value, power, toughness, or card type with each other. Target opponent chooses two. Put chosen into graveyard, rest to hand.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{U}").unwrap(),
                    tap: true,
                    // GAP: "Sacrifice another creature (not self)" not in ActivationCost
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: complex_search,
            }),
    )
}

fn complex_search(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Search constraint "cards that don't share mv/power/toughness/type" and
    // opponent-choice-then-split mechanic are not expressible in the Effect catalog.
    Vec::new()
}
