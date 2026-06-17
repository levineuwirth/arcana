//! Nogi, Draco-Zealot — `{1}{R}{R}` 3/3 Legendary Kobold Shaman.
//! "Dragon spells you cast cost {1} less to cast. Whenever Nogi attacks, if you
//! control three or more Dragons, until end of turn, Nogi becomes a Dragon with
//! base power and toughness 5/5 and gains flying."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::conditions;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nogi, Draco-Zealot");
    let kobold = reg.interner_mut().intern("Kobold");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kobold);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: "Dragon spells you cast cost {1} less" — static cost reduction has
        // no expressible Effect/ability here.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: Some(if_control_three_dragons),
            effect: become_dragon,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn if_control_three_dragons(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    conditions::you_control_at_least(s, you, &script::subtype_filter(reg, "Dragon"), 3)
}

fn become_dragon(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::SetBasePT {
            target: trig.source,
            power: 5,
            toughness: 5,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Flying,
            duration: Duration::EndOfTurn,
        },
        // GAP: "becomes a Dragon" (adds the Dragon creature SUBTYPE) — AddType only
        // adds card types (Artifact/Creature/etc.), not creature subtypes.
    ]
}
