//! Rappelling Scouts — `{2}{W}{W}` 1/4 Human Rebel Scout with Flying.
//! "{2}{W}: This creature gains protection from the color of your
//! choice until end of turn."
//!
//! Flying is a base keyword. The activated ability's effect (grant
//! protection from a chosen color) has no expressible primitive in the
//! provided catalog (Protection is not a usable KeywordAbility, and
//! there is no choose-color-then-grant-protection Effect), so the
//! effect body is GAP'd while the cost-bearing ability is still wired.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rappelling Scouts");
    let human = reg.interner_mut().intern("Human");
    let rebel = reg.interner_mut().intern("Rebel");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rebel);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{W}: This creature gains protection from the color of your choice until end of turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{W}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: gain_protection,
        }),
    )
}

fn gain_protection(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: gain protection from a chosen color — Protection is not a
    // usable KeywordAbility and no choose-color-then-grant Effect exists.
    Vec::new()
}
