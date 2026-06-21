//! Mijo, the Bull — `{2}{R}` Legendary 2/3 Ox Athlete.
//! "When Mijo enters, create a colorless Equipment artifact token named
//! Rock with [granted ability] and equip {1}."
//! "From Downtown — If a source you control would deal exactly 2 damage
//! to a permanent or player, it deals 3 damage instead." (replacement
//! static — GAP).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mijo, the Bull");
    let ox = reg.interner_mut().intern("Ox");
    let athlete = reg.interner_mut().intern("Athlete");
    // Pre-intern the token's name + subtype so the resolver can recover them.
    let _rock = reg.interner_mut().intern("Rock");
    let _equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ox);
    subtypes.0.insert(athlete);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    // GAP: static replacement "From Downtown — exactly-2 damage becomes 3" is
    // not expressible (no damage-amount replacement primitive here).
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: make_rock,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_rock(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let token_name = reg.interner().lookup("Rock").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    if let Some(eq) = reg.interner().lookup("Equipment") {
        subtypes.0.insert(eq);
    }
    // Token-fidelity GAP: the Rock's granted "{1},{T},Sacrifice: deal 2 to any
    // target" ability and its equip {1} cost are not expressible on a token;
    // mints the bare colorless Equipment artifact token named Rock.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: token_name,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes,
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
