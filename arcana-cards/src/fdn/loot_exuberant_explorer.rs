//! Loot, Exuberant Explorer — `{2}{G}` 1/4 Legendary Beast Noble.
//!
//! * "You may play an additional land on each of your turns." — a
//!   static extra-land-play permission with no engine primitive; GAP'd.
//! * "{4}{G}{G}, {T}: Look at the top six cards of your library. You may
//!   reveal a creature card with mana value less than or equal to the
//!   number of lands you control from among them and put it onto the
//!   battlefield. Put the rest on the bottom in a random order." —
//!   modeled with RevealUntil (creature card → battlefield, rest to the
//!   bottom in random order, capped at six reveals).

use arcana_core::effects::{DigRest, Effect, RevealDest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Loot, Exuberant Explorer");
    let beast = reg.interner_mut().intern("Beast");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static "You may play an additional land on each of your turns."
    // has no engine primitive.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{4}{G}{G}, {T}: Look at the top six cards of your library. You may reveal a creature card with mana value less than or equal to the number of lands you control from among them and put it onto the battlefield. Put the rest on the bottom in a random order.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{4}{G}{G}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: dig_for_creature,
        }),
    )
}

fn dig_for_creature(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the "mana value <= number of lands you control" gate and the
    // "look at six with an optional pick" choice are fidelity gaps —
    // RevealUntil takes the first creature card deterministically.
    vec![Effect::RevealUntil {
        player: ctx.controller,
        filter: ObjectFilter::creature(),
        found_dest: RevealDest::Battlefield,
        rest: DigRest::BottomRandom,
        max_reveal: Some(6),
    }]
}
