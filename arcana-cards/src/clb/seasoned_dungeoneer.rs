//! Seasoned Dungeoneer — `{3}{W}` 3/4 Human Warrior.
//!
//! "When this creature enters, you take the initiative." (GAP — no
//! take-the-initiative effect/dungeon-initiative API documented)
//! "Whenever you attack, target attacking Cleric, Rogue, Warrior, or Wizard
//! gains protection from creatures until end of turn. It explores."
//!
//! Scryfall lists "Explore" as a keyword tag; it is the explore mechanic,
//! not a `KeywordAbility` variant, so the keyword line is empty.
//!
//! The second ability is implemented as a CreatureAttacks trigger (you
//! control, one of the four subtypes) targeting that attacker and having it
//! explore. Two fidelity gaps: (1) "protection from creatures" is GAP'd —
//! Protection is not a documented grantable keyword and ProtectionQuality is
//! outside the documented API; (2) "whenever you attack" is approximated by
//! per-attacker CreatureAttacks (may fire once per qualifying attacker).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Seasoned Dungeoneer");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let cleric = reg.interner_mut().intern("Cleric");
    let rogue = reg.interner_mut().intern("Rogue");
    let warrior_s = reg.interner_mut().intern("Warrior");
    let wizard = reg.interner_mut().intern("Wizard");
    let attacker_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![cleric, rogue, warrior_s, wizard]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: take_initiative,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: attacker_filter,
                },
                intervening_if: None,
                effect: explore_attacker,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn take_initiative(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you take the initiative" — no documented take-the-initiative /
    // dungeon-initiative effect.
    Vec::new()
}

fn explore_attacker(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "gains protection from creatures until end of turn" — Protection is
    // not a documented grantable keyword. The explore half is faithful.
    vec![Effect::Explore { player: trig.controller, target: *id }]
}
