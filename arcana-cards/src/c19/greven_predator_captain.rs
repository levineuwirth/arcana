//! Greven, Predator Captain — `{3}{B}{R}` 5/5 Legendary Phyrexian Human
//! Warrior with Menace.
//!
//! * Menace — base keyword.
//! * "Greven gets +X/+0, where X is the amount of life you've lost this turn."
//!   — a static, self-scaling continuous pump with no trigger word or cost;
//!   not expressible as a triggered/activated ability, so it is GAP'd.
//! * "Whenever Greven attacks, you may sacrifice another creature. If you do,
//!   you draw cards equal to that creature's power and you lose life equal to
//!   that creature's toughness." — the sacrifice is implemented (best-effort);
//!   the "you may" optionality and the power/toughness-scaled draw/loss riders
//!   are GAP'd because `Effect::Sacrifice` does not surface the sacrificed
//!   creature's id (Doomgape pattern).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Greven, Predator Captain");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    // GAP: static "Greven gets +X/+0, where X is the life you've lost this turn."
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attacks_sacrifice,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attacks_sacrifice(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may" optionality and "draw cards equal to that creature's power
    // / lose life equal to its toughness" — the sacrificed creature's id is not
    // available from Effect::Sacrifice, so the scaled riders are omitted.
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter::creature(),
        count: 1,
    }]
}
