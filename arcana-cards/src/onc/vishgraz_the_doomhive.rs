//! Vishgraz, the Doomhive — `{2}{W}{B}{G}` 3/3 Legendary Phyrexian Insect.
//!
//! * Menace, Toxic 1 (keywords).
//! * ETB: create three 1/1 colorless Phyrexian Mite artifact creature tokens
//!   with toxic 1. (The token's "This token can't block" clause is a static
//!   ability on a token and is not expressible — GAP'd, but the tokens and
//!   their Toxic 1 keyword are emitted.)
//! * GAP (static): "Vishgraz gets +1/+1 for each poison counter your
//!   opponents have." — a continuous self-pump static, not a trigger/activated
//!   ability, and no poison-counter board read is available here.

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
    let name = reg.interner_mut().intern("Vishgraz, the Doomhive");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(insect);

    // Pre-intern the token subtype so the resolver can look it up.
    let _mite = reg.interner_mut().intern("Mite");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace, KeywordAbility::Toxic(1)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_make_mites,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_make_mites(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mite = reg.interner().lookup("Mite").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mite);
    let token = TokenDefinition {
        name: mite,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Toxic(1)],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token },
    ]
}
