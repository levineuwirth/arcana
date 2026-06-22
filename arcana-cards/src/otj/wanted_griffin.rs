//! Wanted Griffin — `{3}{W}` 3/2 Griffin with Flying.
//! "When this creature dies, create a 1/1 red Mercenary creature token with
//!  '{T}: Target creature you control gets +1/+0 until end of turn. Activate
//!  only as a sorcery.'"
//!
//! Flying is a base keyword. The death trigger creates a 1/1 red Mercenary
//! token. Fidelity gap: TokenDefinition.abilities only holds triggered
//! abilities, so the Mercenary's printed {T}: activated ability can't be
//! embedded on the token — the token is minted with its bones only.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Wanted Griffin");
    let griffin = reg.interner_mut().intern("Griffin");
    let _mercenary = reg.interner_mut().intern("Mercenary");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(griffin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: make_mercenary,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_mercenary(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mercenary = match reg.interner().lookup("Mercenary") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mercenary);
    // GAP: the Mercenary token's printed activated ability ("{T}: Target
    // creature you control gets +1/+0 ... sorcery speed") can't be embedded;
    // TokenDefinition.abilities only accepts triggered abilities.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: mercenary,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
