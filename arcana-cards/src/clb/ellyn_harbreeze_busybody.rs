//! Ellyn Harbreeze, Busybody — `{3}{W}` 2/4 Legendary white Human Peasant.
//!
//! Oracle text:
//! * "{T}: Look at the top X cards of your library, where X is the number of
//!   tokens you created this turn. Put one of those cards into your hand and
//!   the rest on the bottom of your library in a random order."
//! * "Choose a Background"
//!
//! Implementation notes:
//! * The `{T}` ability is a `DigTopN` shape (look at top X, take one to hand,
//!   rest to bottom random), but X = "tokens you created this turn" has no
//!   `script::*` helper to compute it. Per the dynamic-amount rule, the whole
//!   effect is GAP'd rather than emitted with a wrong literal count.
//!   // GAP: effect — no helper for "tokens created this turn"; dynamic X
//!   uncomputable.
//! * Choose a Background is not in the usable keyword surface.
//!   // GAP: keyword — Choose a Background not modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ellyn Harbreeze, Busybody");
    let human = reg.interner_mut().intern("Human");
    let peasant = reg.interner_mut().intern("Peasant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(peasant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Look at the top X cards of your library, where X is the number of tokens you created this turn. Put one of those cards into your hand and the rest on the bottom of your library in a random order.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: dig_x,
        }),
    )
}

fn dig_x(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: effect — X = "tokens you created this turn" has no script helper;
    // dynamic count is uncomputable, so the dig is omitted entirely.
    Vec::new()
}
