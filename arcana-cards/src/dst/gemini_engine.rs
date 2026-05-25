//! Gemini Engine — `{6}` 3/4 colorless Artifact Creature — Construct.
//! "Whenever this creature attacks, create a colorless Construct artifact
//! creature token named Twin that's attacking. Its power is equal to this
//! creature's power and its toughness is equal to this creature's toughness.
//! Sacrifice the token at end of combat."
//! GAP: effect — token P/T equals source's P/T (dynamic); "attacking" state
//! not expressible with CreateToken; sacrifice at end of combat uses
//! DelayedAction::Sacrifice + NextEndStep as approximation.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gemini Engine");
    let twin_name = reg.interner_mut().intern("Twin");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attacks(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let construct = reg.interner().lookup("Construct")
        .expect("Construct interned during register()");
    let twin = reg.interner().lookup("Twin")
        .expect("Twin interned during register()");
    let pwr = script::power_of(state, trig.source).max(0);
    let tgh = script::toughness_of(state, trig.source).max(0);
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(construct);
    let token = TokenDefinition {
        name: twin,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(pwr)),
        toughness: Some(PtValue::Fixed(tgh)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: token is not "attacking"; sacrifice at end of combat uses
    // CreateTokenSacEot as best-effort (sacrifices at next end step).
    vec![Effect::CreateTokenSacEot { controller: trig.controller, token }]
}
