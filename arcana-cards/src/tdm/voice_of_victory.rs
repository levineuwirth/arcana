//! Voice of Victory — `{1}{W}` 1/3 Human Bard.
//!
//! Oracle:
//! * Mobilize 2 (Whenever this creature attacks, create two tapped and attacking
//!   1/1 red Warrior creature tokens. Sacrifice them at the beginning of the next
//!   end step.) — Mobilize has no `KeywordAbility` variant; decomposed into a
//!   SelfAttacks trigger that creates two 1/1 red Warrior tokens via
//!   `CreateTokenSacEot` (created + sacrificed at the next end step). Fidelity
//!   gap: the tokens aren't created tapped and attacking (no tapped-attacking
//!   token primitive in this catalog).
//! * "Your opponents can't cast spells during your turn." — GAP: a static
//!   cast-restriction (no trigger word, no cost); no API surface.

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
    let name = reg.interner_mut().intern("Voice of Victory");
    let human = reg.interner_mut().intern("Human");
    let bard = reg.interner_mut().intern("Bard");
    let _warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(bard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Mobilize 2 keyword line — no KeywordAbility::Mobilize (modeled as
        //      the attacks trigger below).
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static "Your opponents can't cast spells during your turn."
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: mobilize_two,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn mobilize_two(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let warrior = match reg.interner().lookup("Warrior") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(warrior);
    let token = TokenDefinition {
        name: warrior,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateTokenSacEot {
            controller: trig.controller,
            token: token.clone(),
        },
        Effect::CreateTokenSacEot {
            controller: trig.controller,
            token,
        },
    ]
}
