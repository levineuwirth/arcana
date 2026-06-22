//! Scion Summoner — `{2}{G}` 2/2 Eldrazi Drone (Devoid → colorless).
//!
//! Oracle:
//! * Devoid (this card has no color).
//! * When this creature enters, create a 1/1 colorless Eldrazi Scion creature
//!   token. It has "Sacrifice this token: Add {C}."
//!
//! Devoid is not a usable KeywordAbility; it is reflected by the colorless
//! color identity. The token is minted, but its activated mana ability
//! ("Sacrifice this token: Add {C}") cannot be attached — TokenDefinition.
//! abilities holds only triggered abilities, not activated ones — so that
//! ability is GAP'd (fidelity gap on the token).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scion Summoner");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let drone = reg.interner_mut().intern("Drone");
    let _scion = reg.interner_mut().intern("Eldrazi Scion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(drone);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_scion,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_scion(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let scion = match reg.interner().lookup("Eldrazi Scion") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scion);
    // GAP: the token's "Sacrifice this token: Add {C}" activated mana ability
    // can't be attached (TokenDefinition.abilities holds only triggered
    // abilities). The bare 1/1 colorless Eldrazi Scion is minted.
    let token = TokenDefinition {
        name: scion,
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
