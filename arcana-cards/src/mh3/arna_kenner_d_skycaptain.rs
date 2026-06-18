//! Arna Kennerüd, Skycaptain — `{2}{W}{U}{B}` 4/4 Legendary Human Knight
//! (white/blue/black) with Flying and Lifelink.
//!
//! Oracle:
//! * Flying, lifelink.
//! * Ward—Discard a card.
//! * Whenever a modified creature you control attacks, double the number of each
//!   kind of counter on it. Then for each nontoken permanent attached to it,
//!   create a token that's a copy of that permanent attached to that creature.
//!
//! Flying + Lifelink are base keyword characteristics. The remaining text is
//! GAP'd:
//! * Ward—Discard a card is a NON-MANA ward; `KeywordAbility::Ward` only carries
//!   a `ManaCost`, so this is not expressible (kept out of `keywords`).
//! * The attack trigger needs a "modified creature" attacker filter, counter-
//!   doubling of every kind, and per-attached-permanent token copies that enter
//!   already attached — none of which are demonstrated primitives. The trigger
//!   condition is wired (CreatureAttacks) but the effect is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arna Kennerüd, Skycaptain");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: "Ward—Discard a card" — KeywordAbility::Ward only carries a
        // ManaCost; a non-mana ward cost is not expressible, so Ward is omitted.
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: on_modified_attacker,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_modified_attacker(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: requires a "modified creature" attacker restriction, doubling the
    // count of EACH kind of counter on it, and creating token copies of each
    // nontoken permanent attached to it that enter already attached — none of
    // these are demonstrated primitives.
    Vec::new()
}
