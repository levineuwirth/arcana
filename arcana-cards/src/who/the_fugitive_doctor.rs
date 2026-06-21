//! The Fugitive Doctor — `{3}{R}{G}` 4/4 Legendary Time Lord Doctor.
//! "When The Fugitive Doctor enters, investigate." — wired as creating
//! one Clue token (Scryfall's "Investigate" keyword has no KeywordAbility
//! variant, but the effect maps to a Clue commodity token).
//! "Whenever The Fugitive Doctor attacks, you may sacrifice a Clue. When
//! you do, target instant or sorcery card in your graveyard gains
//! flashback {2}{R}{G} until end of turn." — modeled with the
//! grant-flashback-to-a-graveyard-instant-or-sorcery primitive (the
//! controller picks the card). FIDELITY GAP: the "you may sacrifice a
//! Clue" reflexive cost gate is not modeled, and the granted flashback
//! cost is the card's own mana cost rather than the printed {2}{R}{G}.

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("The Fugitive Doctor");
    let timelord = reg.interner_mut().intern("Time Lord");
    let doctor = reg.interner_mut().intern("Doctor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(timelord);
    subtypes.0.insert(doctor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_investigate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_flashback,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_investigate(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Clue,
        count: 1,
    }]
}

fn attack_flashback(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GrantFlashbackToInstantOrSorceryInGraveyard {
        source: trig.source,
        controller: trig.controller,
        duration: Duration::EndOfTurn,
    }]
}
