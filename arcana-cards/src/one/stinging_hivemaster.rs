//! Stinging Hivemaster — `{2}{B}` 3/2 Phyrexian Warlock with Toxic 1.
//!
//! Oracle:
//! * Toxic 1
//! * When this creature dies, create a 1/1 colorless Phyrexian Mite
//!   artifact creature token with toxic 1 and "This token can't block."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Stinging Hivemaster");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let warlock = reg.interner_mut().intern("Warlock");
    let _mite = reg.interner_mut().intern("Mite");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Toxic(1)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: make_mite,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_mite(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mite = match reg.interner().lookup("Mite") {
        Some(sym) => sym,
        None => return Vec::new(),
    };
    let phyrexian = reg.interner().lookup("Phyrexian");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mite);
    if let Some(p) = phyrexian {
        subtypes.0.insert(p);
    }
    // GAP: the token's printed "This token can't block." ability is not
    // expressible as a baked-in token ability; the Toxic 1 keyword and
    // the 1/1 colorless Phyrexian Mite artifact-creature body are wired.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: mite,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Toxic(1)],
            abilities: vec![],
        },
    }]
}
