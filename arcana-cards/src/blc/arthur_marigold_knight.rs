//! Arthur, Marigold Knight — `{2}{U}{R}{W}` 4/5 Legendary Mouse Knight
//! with Haste. "Whenever Arthur and at least one other creature
//! attack, look at the top six cards of your library, put a creature
//! card onto the battlefield tapped and attacking, …" — the
//! look-at-top-N-then-put-onto-battlefield-tapped-and-attacking effect
//! (and the end-of-combat bounce) is not expressible from a library
//! dig, so the trigger body is a gap.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Arthur, Marigold Knight");
    let mouse = reg.interner_mut().intern("Mouse");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mouse);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: "Arthur and at least one other creature attack" — the
            // "at least one other" co-attack condition is not
            // expressible; closest is SelfAttacks.
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: arthur_attack,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn arthur_attack(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "look at the top six, put a creature card onto the
    // battlefield tapped and attacking, rest on bottom, return it at
    // end of combat" — no look-at-top-N → battlefield-tapped-attacking
    // effect (PutFromHandOntoBattlefieldTappedAttacking is hand-only),
    // and no end-of-combat bounce. Omitted.
    Vec::new()
}
