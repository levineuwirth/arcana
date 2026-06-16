//! Veronica, Dissident Scribe — `{2}{R}` 3/3 Legendary Human Artificer Rogue (red).
//! Menace.
//! "Whenever Veronica attacks, you may discard a card. If you do, draw a card." (GAP)
//! "Whenever you discard one or more nonland cards for the first time each turn,
//!  create a Junk token." (token minted without its activated ability — GAP)

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Veronica, Dissident Scribe");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let rogue = reg.interner_mut().intern("Rogue");
    let junk = reg.interner_mut().intern("Junk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    let _ = junk;

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_loot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_junk,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_loot(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may discard a card. If you do, draw a card." — optional discard
    // (a chosen-card discard from hand as a may-cost) is not expressible as an
    // Effect; there is no "you may discard, if you do" primitive.
    Vec::new()
}

fn make_junk(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // The Junk token's printed "{T}, Sacrifice: exile top card, play it"
    // activated ability is not wirable on a token here — GAP that ability,
    // mint the bare artifact token.
    let junk_name = reg.interner().lookup("Junk").unwrap_or_default();
    let junk_sub = reg.interner().lookup("Junk").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(junk_sub);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: junk_name,
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
