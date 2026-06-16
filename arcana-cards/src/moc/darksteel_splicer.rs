//! Darksteel Splicer — `{6}{W}` 1/1 white Phyrexian Artificer.
//! "Whenever this creature or another nontoken Phyrexian you control enters,
//! create X 3/3 colorless Phyrexian Golem artifact creature tokens, where X is
//! the number of opponents you have."
//! GAP: "Golems you control have indestructible." — a static keyword grant to
//! other permanents; not expressible.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Darksteel Splicer");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let artificer = reg.interner_mut().intern("Artificer");
    let _golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    let phyrexian_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .nontoken()
        .with_subtype_sym(phyrexian);
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: phyrexian_filter,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: make_golems,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_golems(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let golem = reg.interner().lookup("Golem").unwrap_or_default();
    let phyrexian = reg.interner().lookup("Phyrexian").unwrap_or_default();
    let n = script::opponents(state, trig.controller).len() as u32;
    let mut out = Vec::new();
    for _ in 0..n {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(golem);
        subtypes.0.insert(phyrexian);
        out.push(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: golem,
                colors: ColorSet::colorless(),
                types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
                subtypes,
                power: Some(PtValue::Fixed(3)),
                toughness: Some(PtValue::Fixed(3)),
                keywords: vec![],
                abilities: vec![],
            },
        });
    }
    out
}
