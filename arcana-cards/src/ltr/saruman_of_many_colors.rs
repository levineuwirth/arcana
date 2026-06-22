//! Saruman of Many Colors — `{3}{W}{U}{B}` 5/4 Legendary Avatar Wizard.
//!
//! Oracle: "Ward—Discard an enchantment, instant, or sorcery card.
//! Whenever you cast your second spell each turn, each opponent mills two
//! cards. When one or more cards are milled this way, exile target
//! enchantment, instant, or sorcery card with equal or lesser mana value
//! than that spell from an opponent's graveyard. Copy the exiled card. You
//! may cast the copy without paying its mana cost."
//!
//! Ward with a non-mana (discard) cost is not expressible (only mana Ward
//! is supported), so the keyword is omitted. The Scryfall "Mill" tag is
//! reminder for the trigger's mill, not a standalone keyword.
//!
//! The "second spell each turn" trigger is wired as a SpellCast(you) trigger
//! gated by an intervening-if that the count of spells you've cast this turn
//! equals two. Its first sentence — each opponent mills two — is expressed.
//! The reflexive "when cards are milled this way, exile target ... copy ...
//! cast the copy" rider is a linked secondary trigger with a graveyard-target
//! exile and free-cast-of-copy; that chain is GAP'd.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Saruman of Many Colors");
    let avatar = reg.interner_mut().intern("Avatar");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: "Ward—Discard an enchantment, instant, or sorcery card" — only
        // mana Ward is expressible; a non-mana (discard) Ward cost is omitted.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::You,
            },
            intervening_if: Some(if_second_spell),
            effect: each_opponent_mills_two,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn if_second_spell(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    // "your second spell each turn" — fires exactly when the spell just cast
    // is the second you've cast this turn.
    script::spells_cast_this_turn(s, &ObjectFilter::new(), you) == 2
}

fn each_opponent_mills_two(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "each opponent mills two cards."
    // GAP: the reflexive "When one or more cards are milled this way, exile
    // target enchantment/instant/sorcery card with mv <= that spell from an
    // opponent's graveyard, copy it, and cast the copy for free" rider — a
    // linked secondary trigger with a graveyard exile target and a free-cast
    // of a card copy; not expressible here.
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::Mill { player: p, count: 2 })
        .collect()
}
