//! Crimson Caravaneer — `{2}{R}` 1/2 Creature — Human Scout.
//! Double strike, trample.
//! Whenever this creature deals combat damage to a player, create a Junk
//! token.
//!
//! The Junk token's printed activated ability ("{T}, Sacrifice this token:
//! Exile the top card of your library. You may play that card this turn.")
//! is not expressible on a TokenDefinition — the bare artifact token is
//! minted and its activation is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crimson Caravaneer");
    let human = reg.interner_mut().intern("Human");
    let scout = reg.interner_mut().intern("Scout");
    // Intern the token name so the resolver can look it up via `reg`.
    let _junk = reg.interner_mut().intern("Junk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::DoubleStrike, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: make_junk,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_junk(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let junk = match reg.interner().lookup("Junk") {
        Some(sym) => sym,
        None => return Vec::new(),
    };
    let token = TokenDefinition {
        name: junk,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes: SubtypeSet::default(),
        power: None,
        toughness: None,
        keywords: vec![],
        // GAP: Junk's "{T}, Sacrifice: exile top card, may play it this turn"
        // activated ability is not expressible on a TokenDefinition.
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
